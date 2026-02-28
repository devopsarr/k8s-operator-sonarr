use std::sync::Arc;

use kube::api::{Api, Patch, PatchParams};
use kube::runtime::controller::Action;
use kube::{Client, ResourceExt};
use tracing::{info, warn};

use sonarr::apis::{series_api, series_lookup_api};
use sonarr::models::{AddSeriesOptions, MonitorTypes, SeriesTypes};

use crate::Context;
use crate::crds::series::{MonitorType, SeriesType};
use crate::crds::{SonarrSeries, SonarrSeriesStatus};
use crate::error::{Error, Result};

use super::tag::get_sonarr_config;
use super::traits::{REQUEUE_DURATION, reconcile_with_finalizer, run_controller};
use super::{ready_condition, update_conditions};

/// Start the SonarrSeries controller
pub async fn run(client: Client, context: Arc<Context>) {
    run_controller::<SonarrSeries, _, _>(client, context, "SonarrSeries", reconcile).await;
}

async fn reconcile(obj: Arc<SonarrSeries>, ctx: Arc<Context>) -> Result<Action> {
    reconcile_with_finalizer(obj, ctx, reconcile_apply, reconcile_cleanup).await
}

async fn reconcile_apply(series: Arc<SonarrSeries>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = series
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;
    let name = series.name_any();

    info!("Reconciling SonarrSeries: {}/{}", namespace, name);

    let config =
        get_sonarr_config(&ctx, client, &namespace, &series.spec.sonarr_instance_ref).await?;

    // Check if series exists by ID or by TVDB ID
    let existing_series = if let Some(id) = series.status.as_ref().and_then(|s| s.id) {
        series_api::get_series_by_id(&config, id, Some(false))
            .await
            .ok()
    } else {
        // Try to find by TVDB ID
        let all_series =
            series_api::list_series(&config, Some(series.spec.tvdb_id), Some(true)).await?;
        all_series
            .into_iter()
            .find(|s| s.tvdb_id == Some(series.spec.tvdb_id))
    };

    let sonarr_series = if let Some(mut existing) = existing_series {
        // Update existing series
        existing.monitored = Some(series.spec.monitored);
        // Use quality_profile.id if available
        if let Some(id) = series.spec.quality_profile.id {
            existing.quality_profile_id = Some(id);
        }
        existing.root_folder_path = Some(Some(series.spec.root_folder_path.clone()));
        existing.season_folder = Some(series.spec.season_folder);
        existing.tags = Some(Some(series.spec.tags.clone()));
        existing.series_type = Some(convert_series_type(&series.spec.series_type));

        series_api::update_series(
            &config,
            existing.id.unwrap_or(0).to_string().as_str(),
            Some(false),
            Some(existing),
        )
        .await?
    } else {
        // Need to lookup series from TVDB first
        let lookup_results = series_lookup_api::list_series_lookup(
            &config,
            Some(&format!("tvdb:{}", series.spec.tvdb_id)),
        )
        .await?;

        if lookup_results.is_empty() {
            return Err(Error::Other(format!(
                "Series with TVDB ID {} not found",
                series.spec.tvdb_id
            )));
        }

        let mut new_series = lookup_results.into_iter().next().unwrap();
        new_series.monitored = Some(series.spec.monitored);
        // Use quality_profile.id if available
        if let Some(id) = series.spec.quality_profile.id {
            new_series.quality_profile_id = Some(id);
        }
        new_series.root_folder_path = Some(Some(series.spec.root_folder_path.clone()));
        new_series.season_folder = Some(series.spec.season_folder);
        new_series.tags = Some(Some(series.spec.tags.clone()));
        new_series.series_type = Some(convert_series_type(&series.spec.series_type));

        // Set add options
        let mut add_options = AddSeriesOptions::new();
        add_options.monitor = Some(convert_monitor_type(&series.spec.add_options.monitor));
        add_options.search_for_missing_episodes =
            Some(series.spec.add_options.search_for_missing_episodes);
        add_options.search_for_cutoff_unmet_episodes =
            Some(series.spec.add_options.search_for_cutoff_unmet_episodes);

        new_series.add_options = Some(Box::new(add_options));

        series_api::create_series(&config, Some(new_series)).await?
    };

    // Update status
    let series_api_k8s: Api<SonarrSeries> = Api::namespaced(client.clone(), &namespace);
    let mut conditions = series
        .status
        .as_ref()
        .map(|s| s.conditions.clone())
        .unwrap_or_default();
    update_conditions(
        &mut conditions,
        ready_condition(true, "Synced", "Series synchronized with Sonarr"),
    );

    // Extract statistics from the series
    let stats = sonarr_series.statistics.as_ref();

    let status = SonarrSeriesStatus {
        conditions,
        id: sonarr_series.id,
        observed_generation: series.metadata.generation.unwrap_or(0),
        episode_count: stats.and_then(|s| s.episode_count),
        episode_file_count: stats.and_then(|s| s.episode_file_count),
        percent_complete: stats.and_then(|s| s.percent_of_episodes),
        next_airing: sonarr_series.next_airing.flatten().map(|d| d.to_string()),
        previous_airing: sonarr_series
            .previous_airing
            .flatten()
            .map(|d| d.to_string()),
        network: sonarr_series.network.flatten(),
        series_status: sonarr_series.status.map(|s| format!("{:?}", s)),
    };

    let status_patch = serde_json::json!({ "status": status });
    series_api_k8s
        .patch_status(&name, &PatchParams::default(), &Patch::Merge(&status_patch))
        .await?;

    Ok(Action::requeue(REQUEUE_DURATION))
}

fn convert_monitor_type(monitor: &MonitorType) -> MonitorTypes {
    match monitor {
        MonitorType::All => MonitorTypes::All,
        MonitorType::Future => MonitorTypes::Future,
        MonitorType::Missing => MonitorTypes::Missing,
        MonitorType::Existing => MonitorTypes::Existing,
        MonitorType::FirstSeason => MonitorTypes::FirstSeason,
        MonitorType::LastSeason => MonitorTypes::LastSeason,
        MonitorType::Recent => MonitorTypes::Recent,
        MonitorType::Pilot => MonitorTypes::Pilot,
        MonitorType::None => MonitorTypes::None,
    }
}

fn convert_series_type(series_type: &SeriesType) -> SeriesTypes {
    match series_type {
        SeriesType::Standard => SeriesTypes::Standard,
        SeriesType::Daily => SeriesTypes::Daily,
        SeriesType::Anime => SeriesTypes::Anime,
    }
}

async fn reconcile_cleanup(series: Arc<SonarrSeries>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = series
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;

    info!(
        "Cleaning up SonarrSeries: {}/{}",
        namespace,
        series.name_any()
    );

    if let Some(id) = series.status.as_ref().and_then(|s| s.id)
        && let Ok(config) =
            get_sonarr_config(&ctx, client, &namespace, &series.spec.sonarr_instance_ref).await
    {
        // Default to not deleting files on cleanup
        warn!("Deleting series {} from Sonarr", id);
        let _ = series_api::delete_series(&config, id, Some(false), Some(false)).await;
    }

    Ok(Action::await_change())
}
