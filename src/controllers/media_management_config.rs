//! Controller for SonarrMediaManagementConfig
//!
//! This controller manages media management configuration for Sonarr instances.
//! Only one SonarrMediaManagementConfig per Sonarr instance is allowed.

use std::sync::Arc;

use kube::api::{Api, ListParams, Patch, PatchParams};
use kube::runtime::controller::Action;
use kube::{Client, ResourceExt};
use tracing::{info, warn};

use sonarr::apis::media_management_config_api;
use sonarr::models::{
    EpisodeTitleRequiredType, FileDateType, MediaManagementConfigResource, ProperDownloadTypes,
    RescanAfterRefreshType,
};

use crate::Context;
use crate::crds::media_management_config::{
    EpisodeTitleRequiredType as CrdEpisodeTitleRequiredType, FileDateType as CrdFileDateType,
    ProperDownloadType, RescanAfterRefreshType as CrdRescanType, SonarrMediaManagementConfig,
    SonarrMediaManagementConfigStatus,
};
use crate::error::{Error, Result};

use super::tag::get_sonarr_config;
use super::traits::{REQUEUE_DURATION, reconcile_with_finalizer, run_controller};
use super::{ready_condition, update_conditions};

/// Start the SonarrMediaManagementConfig controller
pub async fn run(client: Client, context: Arc<Context>) {
    run_controller::<SonarrMediaManagementConfig, _, _>(
        client,
        context,
        "SonarrMediaManagementConfig",
        reconcile,
    )
    .await;
}

async fn reconcile(obj: Arc<SonarrMediaManagementConfig>, ctx: Arc<Context>) -> Result<Action> {
    reconcile_with_finalizer(obj, ctx, reconcile_apply, reconcile_cleanup).await
}

/// Check if another SonarrMediaManagementConfig exists for the same Sonarr instance
async fn check_singleton(
    client: &Client,
    namespace: &str,
    current_name: &str,
    instance_ref_name: &str,
    instance_ref_namespace: Option<&str>,
) -> Result<Option<String>> {
    let api: Api<SonarrMediaManagementConfig> = Api::namespaced(client.clone(), namespace);
    let configs = api.list(&ListParams::default()).await?;

    for config in configs.items {
        let config_name = config.name_any();
        if config_name == current_name {
            continue;
        }

        let ref_name = &config.spec.sonarr_instance_ref.name;
        let ref_ns = config.spec.sonarr_instance_ref.namespace.as_deref();

        if ref_name == instance_ref_name && ref_ns == instance_ref_namespace {
            // Found another config for the same instance
            // Check which one is older
            let current_created = config.metadata.creation_timestamp.as_ref();
            if current_created.is_some() {
                return Ok(Some(config_name));
            }
        }
    }

    Ok(None)
}

async fn reconcile_apply(
    config: Arc<SonarrMediaManagementConfig>,
    ctx: Arc<Context>,
) -> Result<Action> {
    let client = &ctx.client;
    let namespace = config
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;
    let name = config.name_any();

    info!(
        "Reconciling SonarrMediaManagementConfig: {}/{}",
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
            "Another SonarrMediaManagementConfig '{}' already exists for Sonarr instance '{}'. Only one config per instance is allowed.",
            existing_name, config.spec.sonarr_instance_ref.name
        );

        // Update status to show conflict
        let api: Api<SonarrMediaManagementConfig> = Api::namespaced(client.clone(), &namespace);
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

        let status = SonarrMediaManagementConfigStatus {
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
    let existing = media_management_config_api::get_media_management_config(&sonarr_config).await?;

    // Build update resource
    let mut resource = MediaManagementConfigResource::new();
    resource.id = existing.id;

    // Apply settings from CRD, falling back to existing values
    resource.auto_unmonitor_previously_downloaded_episodes = config
        .spec
        .auto_unmonitor_previously_downloaded_episodes
        .or(existing.auto_unmonitor_previously_downloaded_episodes);

    resource.recycle_bin = config
        .spec
        .recycle_bin
        .clone()
        .map(Some)
        .or(existing.recycle_bin);

    resource.recycle_bin_cleanup_days = config
        .spec
        .recycle_bin_cleanup_days
        .or(existing.recycle_bin_cleanup_days);

    resource.download_propers_and_repacks = config
        .spec
        .download_propers_and_repacks
        .as_ref()
        .map(|v| match v {
            ProperDownloadType::DoNotPrefer => ProperDownloadTypes::DoNotPrefer,
            ProperDownloadType::PreferAndUpgrade => ProperDownloadTypes::PreferAndUpgrade,
            ProperDownloadType::DoNotUpgrade => ProperDownloadTypes::DoNotUpgrade,
        })
        .or(existing.download_propers_and_repacks);

    resource.create_empty_series_folders = config
        .spec
        .create_empty_series_folders
        .or(existing.create_empty_series_folders);

    resource.delete_empty_folders = config
        .spec
        .delete_empty_folders
        .or(existing.delete_empty_folders);

    resource.file_date = config
        .spec
        .file_date
        .as_ref()
        .map(|v| match v {
            CrdFileDateType::None => FileDateType::None,
            CrdFileDateType::LocalAirDate => FileDateType::LocalAirDate,
            CrdFileDateType::UtcAirDate => FileDateType::UtcAirDate,
        })
        .or(existing.file_date);

    resource.rescan_after_refresh = config
        .spec
        .rescan_after_refresh
        .as_ref()
        .map(|v| match v {
            CrdRescanType::Always => RescanAfterRefreshType::Always,
            CrdRescanType::AfterManual => RescanAfterRefreshType::AfterManual,
            CrdRescanType::Never => RescanAfterRefreshType::Never,
        })
        .or(existing.rescan_after_refresh);

    resource.set_permissions_linux = config
        .spec
        .set_permissions_linux
        .or(existing.set_permissions_linux);

    resource.chmod_folder = config
        .spec
        .chmod_folder
        .clone()
        .map(Some)
        .or(existing.chmod_folder);

    resource.chown_group = config
        .spec
        .chown_group
        .clone()
        .map(Some)
        .or(existing.chown_group);

    resource.episode_title_required = config
        .spec
        .episode_title_required
        .as_ref()
        .map(|v| match v {
            CrdEpisodeTitleRequiredType::Always => EpisodeTitleRequiredType::Always,
            CrdEpisodeTitleRequiredType::BulkSeasonReleases => {
                EpisodeTitleRequiredType::BulkSeasonReleases
            }
            CrdEpisodeTitleRequiredType::Never => EpisodeTitleRequiredType::Never,
        })
        .or(existing.episode_title_required);

    resource.skip_free_space_check_when_importing = config
        .spec
        .skip_free_space_check_when_importing
        .or(existing.skip_free_space_check_when_importing);

    resource.minimum_free_space_when_importing = config
        .spec
        .minimum_free_space_when_importing
        .or(existing.minimum_free_space_when_importing);

    resource.copy_using_hardlinks = config
        .spec
        .copy_using_hardlinks
        .or(existing.copy_using_hardlinks);

    resource.use_script_import = config.spec.use_script_import.or(existing.use_script_import);

    resource.script_import_path = config
        .spec
        .script_import_path
        .clone()
        .map(Some)
        .or(existing.script_import_path);

    resource.import_extra_files = config
        .spec
        .import_extra_files
        .or(existing.import_extra_files);

    resource.extra_file_extensions = config
        .spec
        .extra_file_extensions
        .clone()
        .map(Some)
        .or(existing.extra_file_extensions);

    resource.enable_media_info = config.spec.enable_media_info.or(existing.enable_media_info);

    // Update config
    let id = existing
        .id
        .ok_or(Error::MissingObjectKey("media_management_config.id"))?;
    media_management_config_api::update_media_management_config(
        &sonarr_config,
        &id.to_string(),
        Some(resource),
    )
    .await?;

    // Update status
    let api: Api<SonarrMediaManagementConfig> = Api::namespaced(client.clone(), &namespace);
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
            "Media management config synchronized with Sonarr",
        ),
    );

    let status = SonarrMediaManagementConfigStatus {
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
    _config: Arc<SonarrMediaManagementConfig>,
    _ctx: Arc<Context>,
) -> Result<Action> {
    // Config settings persist in Sonarr, nothing to clean up
    Ok(Action::await_change())
}
