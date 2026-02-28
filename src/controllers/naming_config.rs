//! Controller for SonarrNamingConfig
//!
//! This controller manages episode naming configuration for Sonarr instances.
//! Only one SonarrNamingConfig per Sonarr instance is allowed.

use std::sync::Arc;

use kube::api::{Api, ListParams, Patch, PatchParams};
use kube::runtime::controller::Action;
use kube::{Client, ResourceExt};
use tracing::{info, warn};

use sonarr::apis::naming_config_api;
use sonarr::models::NamingConfigResource;

use crate::Context;
use crate::crds::naming_config::{SonarrNamingConfig, SonarrNamingConfigStatus};
use crate::error::{Error, Result};

use super::tag::get_sonarr_config;
use super::traits::{REQUEUE_DURATION, reconcile_with_finalizer, run_controller};
use super::{ready_condition, update_conditions};

/// Start the SonarrNamingConfig controller
pub async fn run(client: Client, context: Arc<Context>) {
    run_controller::<SonarrNamingConfig, _, _>(client, context, "SonarrNamingConfig", reconcile)
        .await;
}

async fn reconcile(obj: Arc<SonarrNamingConfig>, ctx: Arc<Context>) -> Result<Action> {
    reconcile_with_finalizer(obj, ctx, reconcile_apply, reconcile_cleanup).await
}

/// Check if another SonarrNamingConfig exists for the same Sonarr instance
async fn check_singleton(
    client: &Client,
    namespace: &str,
    current_name: &str,
    instance_ref_name: &str,
    instance_ref_namespace: Option<&str>,
) -> Result<Option<String>> {
    let api: Api<SonarrNamingConfig> = Api::namespaced(client.clone(), namespace);
    let configs = api.list(&ListParams::default()).await?;

    for config in configs.items {
        let config_name = config.name_any();
        if config_name == current_name {
            continue;
        }

        let ref_name = &config.spec.sonarr_instance_ref.name;
        let ref_ns = config.spec.sonarr_instance_ref.namespace.as_deref();

        if ref_name == instance_ref_name && ref_ns == instance_ref_namespace {
            return Ok(Some(config_name));
        }
    }

    Ok(None)
}

async fn reconcile_apply(config: Arc<SonarrNamingConfig>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = config
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;
    let name = config.name_any();

    info!("Reconciling SonarrNamingConfig: {}/{}", namespace, name);

    // Check singleton constraint
    if let Some(existing_name) = check_singleton(
        client,
        &namespace,
        &name,
        &config.spec.sonarr_instance_ref.name,
        config.spec.sonarr_instance_ref.namespace.as_deref(),
    )
    .await?
    {
        warn!(
            "Another SonarrNamingConfig '{}' already exists for Sonarr instance '{}'. Only one config per instance is allowed.",
            existing_name, config.spec.sonarr_instance_ref.name
        );

        let api: Api<SonarrNamingConfig> = Api::namespaced(client.clone(), &namespace);
        let mut conditions = config
            .status
            .as_ref()
            .map(|s| s.conditions.clone())
            .unwrap_or_default();
        update_conditions(
            &mut conditions,
            ready_condition(
                false,
                "Conflict",
                &format!(
                    "Another config '{}' already exists for this Sonarr instance",
                    existing_name
                ),
            ),
        );

        let status = SonarrNamingConfigStatus {
            conditions,
            observed_generation: config.metadata.generation.unwrap_or(0),
        };

        api.patch_status(
            &name,
            &PatchParams::apply("sonarr-operator"),
            &Patch::Merge(serde_json::json!({ "status": status })),
        )
        .await?;

        return Ok(Action::requeue(REQUEUE_DURATION));
    }

    let sonarr_config =
        get_sonarr_config(&ctx, client, &namespace, &config.spec.sonarr_instance_ref).await?;

    // Get existing config (there's only one, id=1)
    let existing = naming_config_api::get_naming_config(&sonarr_config).await?;

    // Build update resource
    let mut resource = NamingConfigResource::new();
    resource.id = existing.id;

    resource.rename_episodes = config.spec.rename_episodes.or(existing.rename_episodes);

    resource.replace_illegal_characters = config
        .spec
        .replace_illegal_characters
        .or(existing.replace_illegal_characters);

    resource.colon_replacement_format = config
        .spec
        .colon_replacement_format
        .or(existing.colon_replacement_format);

    resource.custom_colon_replacement_format = config
        .spec
        .custom_colon_replacement_format
        .clone()
        .map(Some)
        .or(existing.custom_colon_replacement_format);

    resource.multi_episode_style = config
        .spec
        .multi_episode_style
        .or(existing.multi_episode_style);

    resource.standard_episode_format = config
        .spec
        .standard_episode_format
        .clone()
        .map(Some)
        .or(existing.standard_episode_format);

    resource.daily_episode_format = config
        .spec
        .daily_episode_format
        .clone()
        .map(Some)
        .or(existing.daily_episode_format);

    resource.anime_episode_format = config
        .spec
        .anime_episode_format
        .clone()
        .map(Some)
        .or(existing.anime_episode_format);

    resource.series_folder_format = config
        .spec
        .series_folder_format
        .clone()
        .map(Some)
        .or(existing.series_folder_format);

    resource.season_folder_format = config
        .spec
        .season_folder_format
        .clone()
        .map(Some)
        .or(existing.season_folder_format);

    resource.specials_folder_format = config
        .spec
        .specials_folder_format
        .clone()
        .map(Some)
        .or(existing.specials_folder_format);

    // Update config
    let id = existing
        .id
        .ok_or(Error::MissingObjectKey("naming_config.id"))?;
    naming_config_api::update_naming_config(&sonarr_config, &id.to_string(), Some(resource))
        .await?;

    // Update status
    let api: Api<SonarrNamingConfig> = Api::namespaced(client.clone(), &namespace);
    let mut conditions = config
        .status
        .as_ref()
        .map(|s| s.conditions.clone())
        .unwrap_or_default();
    update_conditions(
        &mut conditions,
        ready_condition(true, "Synced", "Naming config synchronized with Sonarr"),
    );

    let status = SonarrNamingConfigStatus {
        conditions,
        observed_generation: config.metadata.generation.unwrap_or(0),
    };

    api.patch_status(
        &name,
        &PatchParams::apply("sonarr-operator"),
        &Patch::Merge(serde_json::json!({ "status": status })),
    )
    .await?;

    Ok(Action::requeue(REQUEUE_DURATION))
}

async fn reconcile_cleanup(_config: Arc<SonarrNamingConfig>, _ctx: Arc<Context>) -> Result<Action> {
    // Config settings persist in Sonarr, nothing to clean up
    Ok(Action::await_change())
}
