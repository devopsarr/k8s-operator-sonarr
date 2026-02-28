use std::sync::Arc;

use kube::api::{Api, Patch, PatchParams};
use kube::runtime::controller::Action;
use kube::{Client, ResourceExt};
use tracing::info;

use sonarr::apis::quality_profile_api;
use sonarr::models::QualityProfileResource;

use crate::Context;
use crate::crds::{SonarrQualityProfile, SonarrQualityProfileStatus};
use crate::error::{Error, Result};

use super::tag::get_sonarr_config;
use super::traits::{REQUEUE_DURATION, reconcile_with_finalizer, run_controller};
use super::{ready_condition, update_conditions};

/// Start the SonarrQualityProfile controller
pub async fn run(client: Client, context: Arc<Context>) {
    run_controller::<SonarrQualityProfile, _, _>(
        client,
        context,
        "SonarrQualityProfile",
        reconcile,
    )
    .await;
}

async fn reconcile(obj: Arc<SonarrQualityProfile>, ctx: Arc<Context>) -> Result<Action> {
    reconcile_with_finalizer(obj, ctx, reconcile_apply, reconcile_cleanup).await
}

async fn reconcile_apply(profile: Arc<SonarrQualityProfile>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = profile
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;
    let name = profile.name_any();

    info!("Reconciling SonarrQualityProfile: {}/{}", namespace, name);

    let config =
        get_sonarr_config(&ctx, client, &namespace, &profile.spec.sonarr_instance_ref).await?;

    // Build quality profile resource
    let mut qp_resource = QualityProfileResource::new();
    qp_resource.name = Some(Some(profile.spec.name.clone()));
    qp_resource.upgrade_allowed = Some(profile.spec.upgrade_allowed);
    qp_resource.cutoff = Some(profile.spec.cutoff);
    qp_resource.cutoff_format_score = profile.spec.cutoff_format_score;
    qp_resource.min_format_score = profile.spec.min_format_score;
    // Note: Quality groups would need more complex mapping

    let sonarr_profile = if let Some(id) = profile.status.as_ref().and_then(|s| s.id) {
        qp_resource.id = Some(id);
        match quality_profile_api::update_quality_profile(
            &config,
            id.to_string().as_str(),
            Some(qp_resource.clone()),
        )
        .await
        {
            Ok(p) => p,
            Err(_) => {
                qp_resource.id = None;
                quality_profile_api::create_quality_profile(&config, Some(qp_resource)).await?
            }
        }
    } else {
        let existing = quality_profile_api::list_quality_profile(&config).await?;
        if let Some(existing_profile) = existing
            .iter()
            .find(|p| p.name.as_ref().and_then(|n| n.as_ref()) == Some(&profile.spec.name))
        {
            existing_profile.clone()
        } else {
            quality_profile_api::create_quality_profile(&config, Some(qp_resource)).await?
        }
    };

    // Update status
    let profiles_api: Api<SonarrQualityProfile> = Api::namespaced(client.clone(), &namespace);
    let mut conditions = profile
        .status
        .as_ref()
        .map(|s| s.conditions.clone())
        .unwrap_or_default();
    update_conditions(
        &mut conditions,
        ready_condition(true, "Synced", "Quality profile synchronized with Sonarr"),
    );

    let status = SonarrQualityProfileStatus {
        conditions,
        id: sonarr_profile.id,
        observed_generation: profile.metadata.generation.unwrap_or(0),
    };

    let status_patch = serde_json::json!({ "status": status });
    profiles_api
        .patch_status(&name, &PatchParams::default(), &Patch::Merge(&status_patch))
        .await?;

    Ok(Action::requeue(REQUEUE_DURATION))
}

async fn reconcile_cleanup(
    profile: Arc<SonarrQualityProfile>,
    ctx: Arc<Context>,
) -> Result<Action> {
    let client = &ctx.client;
    let namespace = profile
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;

    info!(
        "Cleaning up SonarrQualityProfile: {}/{}",
        namespace,
        profile.name_any()
    );

    if let Some(id) = profile.status.as_ref().and_then(|s| s.id)
        && let Ok(config) =
            get_sonarr_config(&ctx, client, &namespace, &profile.spec.sonarr_instance_ref).await
    {
        let _ = quality_profile_api::delete_quality_profile(&config, id).await;
    }

    Ok(Action::await_change())
}
