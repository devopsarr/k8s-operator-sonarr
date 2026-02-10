//! Controller for SonarrDownloadClientConfig
//!
//! This controller manages global download client configuration for Sonarr instances.
//! Only one SonarrDownloadClientConfig per Sonarr instance is allowed.

use std::sync::Arc;

use kube::api::{Api, ListParams, Patch, PatchParams};
use kube::runtime::controller::Action;
use kube::{Client, ResourceExt};
use tracing::{info, warn};

use sonarr::apis::download_client_config_api;
use sonarr::models::DownloadClientConfigResource;

use crate::Context;
use crate::crds::download_client_config::{
    SonarrDownloadClientConfig, SonarrDownloadClientConfigStatus,
};
use crate::error::{Error, Result};

use super::tag::get_sonarr_config;
use super::traits::{REQUEUE_DURATION, reconcile_with_finalizer, run_controller};
use super::{ready_condition, update_conditions};

/// Start the SonarrDownloadClientConfig controller
pub async fn run(client: Client, context: Arc<Context>) {
    run_controller::<SonarrDownloadClientConfig, _, _>(
        client,
        context,
        "SonarrDownloadClientConfig",
        reconcile,
    )
    .await;
}

async fn reconcile(obj: Arc<SonarrDownloadClientConfig>, ctx: Arc<Context>) -> Result<Action> {
    reconcile_with_finalizer(obj, ctx, reconcile_apply, reconcile_cleanup).await
}

/// Check if another SonarrDownloadClientConfig exists for the same Sonarr instance
async fn check_singleton(
    client: &Client,
    namespace: &str,
    current_name: &str,
    instance_ref_name: &str,
    instance_ref_namespace: Option<&str>,
) -> Result<Option<String>> {
    let api: Api<SonarrDownloadClientConfig> = Api::namespaced(client.clone(), namespace);
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

async fn reconcile_apply(
    config: Arc<SonarrDownloadClientConfig>,
    ctx: Arc<Context>,
) -> Result<Action> {
    let client = &ctx.client;
    let namespace = config
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;
    let name = config.name_any();

    info!(
        "Reconciling SonarrDownloadClientConfig: {}/{}",
        namespace, name
    );

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
            "Another SonarrDownloadClientConfig '{}' already exists for Sonarr instance '{}'. Only one config per instance is allowed.",
            existing_name, config.spec.sonarr_instance_ref.name
        );

        let api: Api<SonarrDownloadClientConfig> = Api::namespaced(client.clone(), &namespace);
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

        let status = SonarrDownloadClientConfigStatus {
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
    let existing =
        download_client_config_api::get_download_client_config(&sonarr_config).await?;

    // Build update resource
    let mut resource = DownloadClientConfigResource::new();
    resource.id = existing.id;

    resource.download_client_working_folders = config
        .spec
        .download_client_working_folders
        .clone()
        .map(Some)
        .or(existing.download_client_working_folders);

    resource.enable_completed_download_handling = config
        .spec
        .enable_completed_download_handling
        .or(existing.enable_completed_download_handling);

    resource.auto_redownload_failed = config
        .spec
        .auto_redownload_failed
        .or(existing.auto_redownload_failed);

    resource.auto_redownload_failed_from_interactive_search = config
        .spec
        .auto_redownload_failed_from_interactive_search
        .or(existing.auto_redownload_failed_from_interactive_search);

    // Update config
    let id = existing
        .id
        .ok_or(Error::MissingObjectKey("download_client_config.id"))?;
    download_client_config_api::update_download_client_config(
        &sonarr_config,
        &id.to_string(),
        Some(resource),
    )
    .await?;

    // Update status
    let api: Api<SonarrDownloadClientConfig> = Api::namespaced(client.clone(), &namespace);
    let mut conditions = config
        .status
        .as_ref()
        .map(|s| s.conditions.clone())
        .unwrap_or_default();
    update_conditions(
        &mut conditions,
        ready_condition(
            true,
            "Synced",
            "Download client config synchronized with Sonarr",
        ),
    );

    let status = SonarrDownloadClientConfigStatus {
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
    _config: Arc<SonarrDownloadClientConfig>,
    _ctx: Arc<Context>,
) -> Result<Action> {
    // Config settings persist in Sonarr, nothing to clean up
    Ok(Action::await_change())
}
