use std::sync::Arc;

use kube::api::{Api, Patch, PatchParams};
use kube::runtime::controller::Action;
use kube::{Client, ResourceExt};
use tracing::info;

use sonarr::apis::delay_profile_api;
use sonarr::models::DelayProfileResource;

use crate::Context;
use crate::crds::delay_profile::DownloadProtocol;
use crate::crds::{SonarrDelayProfile, SonarrDelayProfileStatus};
use crate::error::{Error, Result};

use super::tag::get_sonarr_config;
use super::traits::{REQUEUE_DURATION, reconcile_with_finalizer, run_controller};
use super::{ready_condition, update_conditions};

/// Start the SonarrDelayProfile controller
pub async fn run(client: Client, context: Arc<Context>) {
    run_controller::<SonarrDelayProfile, _, _>(client, context, "SonarrDelayProfile", reconcile)
        .await;
}

async fn reconcile(obj: Arc<SonarrDelayProfile>, ctx: Arc<Context>) -> Result<Action> {
    reconcile_with_finalizer(obj, ctx, reconcile_apply, reconcile_cleanup).await
}

async fn reconcile_apply(profile: Arc<SonarrDelayProfile>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = profile
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;
    let name = profile.name_any();

    info!("Reconciling SonarrDelayProfile: {}/{}", namespace, name);

    let config =
        get_sonarr_config(&ctx, client, &namespace, &profile.spec.sonarr_instance_ref).await?;

    // Build delay profile resource
    let mut resource = DelayProfileResource::new();
    resource.enable_usenet = Some(profile.spec.enable_usenet);
    resource.enable_torrent = Some(profile.spec.enable_torrent);
    resource.preferred_protocol = Some(convert_protocol(&profile.spec.preferred_protocol));
    resource.usenet_delay = Some(profile.spec.usenet_delay);
    resource.torrent_delay = Some(profile.spec.torrent_delay);
    resource.bypass_if_highest_quality = Some(profile.spec.bypass_if_highest_quality);
    resource.bypass_if_above_custom_format_score =
        Some(profile.spec.bypass_if_above_custom_format_score);
    resource.minimum_custom_format_score = Some(profile.spec.minimum_custom_format_score);
    resource.order = Some(profile.spec.order);
    resource.tags = Some(Some(profile.spec.tags.clone()));

    let sonarr_profile = if let Some(id) = profile.status.as_ref().and_then(|s| s.id) {
        resource.id = Some(id);
        match delay_profile_api::update_delay_profile(
            &config,
            &id.to_string(),
            Some(resource.clone()),
        )
        .await
        {
            Ok(p) => p,
            Err(_) => {
                resource.id = None;
                delay_profile_api::create_delay_profile(&config, Some(resource)).await?
            }
        }
    } else {
        // Delay profiles don't have unique names, so we always create new ones
        // unless we have an ID stored
        delay_profile_api::create_delay_profile(&config, Some(resource)).await?
    };

    // Update status
    let api: Api<SonarrDelayProfile> = Api::namespaced(client.clone(), &namespace);
    let mut conditions = profile
        .status
        .as_ref()
        .map(|s| s.conditions.clone())
        .unwrap_or_default();
    update_conditions(
        &mut conditions,
        ready_condition(true, "Synced", "Delay profile synchronized with Sonarr"),
    );

    let status = SonarrDelayProfileStatus {
        conditions,
        id: sonarr_profile.id,
        observed_generation: profile.metadata.generation.unwrap_or(0),
    };

    let status_patch = serde_json::json!({ "status": status });
    api.patch_status(&name, &PatchParams::default(), &Patch::Merge(&status_patch))
        .await?;

    Ok(Action::requeue(REQUEUE_DURATION))
}

fn convert_protocol(protocol: &DownloadProtocol) -> sonarr::models::DownloadProtocol {
    match protocol {
        DownloadProtocol::Usenet => sonarr::models::DownloadProtocol::Usenet,
        DownloadProtocol::Torrent => sonarr::models::DownloadProtocol::Torrent,
    }
}

async fn reconcile_cleanup(profile: Arc<SonarrDelayProfile>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = profile
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;

    info!(
        "Cleaning up SonarrDelayProfile: {}/{}",
        namespace,
        profile.name_any()
    );

    if let Some(id) = profile.status.as_ref().and_then(|s| s.id)
        && let Ok(config) =
            get_sonarr_config(&ctx, client, &namespace, &profile.spec.sonarr_instance_ref).await
    {
        let _ = delay_profile_api::delete_delay_profile(&config, id).await;
    }

    Ok(Action::await_change())
}
