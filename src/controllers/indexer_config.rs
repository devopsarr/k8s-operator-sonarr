//! Controller for SonarrIndexerConfig
//!
//! This controller manages global indexer configuration for Sonarr instances.
//! Only one SonarrIndexerConfig per Sonarr instance is allowed.

use std::sync::Arc;

use kube::api::{Api, ListParams, Patch, PatchParams};
use kube::runtime::controller::Action;
use kube::{Client, ResourceExt};
use tracing::{info, warn};

use sonarr::apis::indexer_config_api;
use sonarr::models::IndexerConfigResource;

use crate::Context;
use crate::crds::indexer_config::{SonarrIndexerConfig, SonarrIndexerConfigStatus};
use crate::error::{Error, Result};

use super::tag::get_sonarr_config;
use super::traits::{REQUEUE_DURATION, reconcile_with_finalizer, run_controller};
use super::{ready_condition, update_conditions};

/// Start the SonarrIndexerConfig controller
pub async fn run(client: Client, context: Arc<Context>) {
    run_controller::<SonarrIndexerConfig, _, _>(client, context, "SonarrIndexerConfig", reconcile)
        .await;
}

async fn reconcile(obj: Arc<SonarrIndexerConfig>, ctx: Arc<Context>) -> Result<Action> {
    reconcile_with_finalizer(obj, ctx, reconcile_apply, reconcile_cleanup).await
}

/// Check if another SonarrIndexerConfig exists for the same Sonarr instance
async fn check_singleton(
    client: &Client,
    namespace: &str,
    current_name: &str,
    instance_ref_name: &str,
    instance_ref_namespace: Option<&str>,
) -> Result<Option<String>> {
    let api: Api<SonarrIndexerConfig> = Api::namespaced(client.clone(), namespace);
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

async fn reconcile_apply(config: Arc<SonarrIndexerConfig>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = config
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;
    let name = config.name_any();

    info!("Reconciling SonarrIndexerConfig: {}/{}", namespace, name);

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
            "Another SonarrIndexerConfig '{}' already exists for Sonarr instance '{}'. Only one config per instance is allowed.",
            existing_name, config.spec.sonarr_instance_ref.name
        );

        let api: Api<SonarrIndexerConfig> = Api::namespaced(client.clone(), &namespace);
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

        let status = SonarrIndexerConfigStatus {
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
    let existing = indexer_config_api::get_indexer_config(&sonarr_config).await?;

    // Build update resource
    let mut resource = IndexerConfigResource::new();
    resource.id = existing.id;

    resource.minimum_age = config.spec.minimum_age.or(existing.minimum_age);
    resource.retention = config.spec.retention.or(existing.retention);
    resource.maximum_size = config.spec.maximum_size.or(existing.maximum_size);
    resource.rss_sync_interval = config.spec.rss_sync_interval.or(existing.rss_sync_interval);

    // Update config
    let id = existing
        .id
        .ok_or(Error::MissingObjectKey("indexer_config.id"))?;
    indexer_config_api::update_indexer_config(&sonarr_config, &id.to_string(), Some(resource))
        .await?;

    // Update status
    let api: Api<SonarrIndexerConfig> = Api::namespaced(client.clone(), &namespace);
    let mut conditions = config
        .status
        .as_ref()
        .map(|s| s.conditions.clone())
        .unwrap_or_default();
    update_conditions(
        &mut conditions,
        ready_condition(true, "Synced", "Indexer config synchronized with Sonarr"),
    );

    let status = SonarrIndexerConfigStatus {
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

async fn reconcile_cleanup(
    _config: Arc<SonarrIndexerConfig>,
    _ctx: Arc<Context>,
) -> Result<Action> {
    // Config settings persist in Sonarr, nothing to clean up
    Ok(Action::await_change())
}
