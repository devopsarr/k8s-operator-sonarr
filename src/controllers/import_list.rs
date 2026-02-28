use std::sync::Arc;

use kube::api::{Api, Patch, PatchParams};
use kube::runtime::controller::Action;
use kube::{Client, ResourceExt};
use tracing::info;

use sonarr::apis::import_list_api;
use sonarr::models::ImportListResource;

use crate::Context;
use crate::crds::import_list::{ImportListType, MonitorTypes, NewItemMonitorTypes, SeriesTypes};
use crate::crds::{SonarrImportList, SonarrImportListStatus};
use crate::error::{Error, Result};

use super::tag::get_sonarr_config;
use super::traits::{REQUEUE_DURATION, reconcile_with_finalizer, run_controller};
use super::{ready_condition, update_conditions};

/// Start the SonarrImportList controller
pub async fn run(client: Client, context: Arc<Context>) {
    run_controller::<SonarrImportList, _, _>(client, context, "SonarrImportList", reconcile).await;
}

async fn reconcile(obj: Arc<SonarrImportList>, ctx: Arc<Context>) -> Result<Action> {
    reconcile_with_finalizer(obj, ctx, reconcile_apply, reconcile_cleanup).await
}

async fn reconcile_apply(import_list: Arc<SonarrImportList>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = import_list
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;
    let name = import_list.name_any();

    info!("Reconciling SonarrImportList: {}/{}", namespace, name);

    let config = get_sonarr_config(
        &ctx,
        client,
        &namespace,
        &import_list.spec.sonarr_instance_ref,
    )
    .await?;

    // Build import list resource
    let mut resource = ImportListResource::new();
    resource.name = Some(Some(import_list.spec.name.clone()));
    resource.implementation = Some(Some(
        get_implementation_name(&import_list.spec.list_type).to_string(),
    ));
    resource.enable_automatic_add = Some(import_list.spec.enable_automatic_add);
    resource.search_for_missing_episodes = Some(import_list.spec.search_for_missing_episodes);
    resource.should_monitor = Some(convert_monitor_type(&import_list.spec.should_monitor));
    resource.monitor_new_items = Some(convert_new_item_monitor_type(
        &import_list.spec.monitor_new_items,
    ));
    resource.root_folder_path = Some(Some(import_list.spec.root_folder_path.clone()));
    resource.quality_profile_id = Some(import_list.spec.quality_profile_id);
    resource.series_type = Some(convert_series_type(&import_list.spec.series_type));
    resource.season_folder = Some(import_list.spec.season_folder);
    resource.list_order = Some(import_list.spec.list_order);
    resource.tags = Some(Some(import_list.spec.tags.clone()));

    let sonarr_import_list = if let Some(id) = import_list.status.as_ref().and_then(|s| s.id) {
        resource.id = Some(id);
        match import_list_api::update_import_list(&config, id, Some(false), Some(resource.clone()))
            .await
        {
            Ok(i) => i,
            Err(_) => {
                resource.id = None;
                import_list_api::create_import_list(&config, Some(false), Some(resource)).await?
            }
        }
    } else {
        let existing = import_list_api::list_import_list(&config).await?;
        if let Some(existing_item) = existing
            .iter()
            .find(|i| i.name.as_ref().and_then(|n| n.as_ref()) == Some(&import_list.spec.name))
        {
            existing_item.clone()
        } else {
            import_list_api::create_import_list(&config, Some(false), Some(resource)).await?
        }
    };

    // Update status
    let api: Api<SonarrImportList> = Api::namespaced(client.clone(), &namespace);
    let mut conditions = import_list
        .status
        .as_ref()
        .map(|s| s.conditions.clone())
        .unwrap_or_default();
    update_conditions(
        &mut conditions,
        ready_condition(true, "Synced", "Import list synchronized with Sonarr"),
    );

    let status = SonarrImportListStatus {
        conditions,
        id: sonarr_import_list.id,
        observed_generation: import_list.metadata.generation.unwrap_or(0),
    };

    let status_patch = serde_json::json!({ "status": status });
    api.patch_status(&name, &PatchParams::default(), &Patch::Merge(&status_patch))
        .await?;

    Ok(Action::requeue(REQUEUE_DURATION))
}

fn get_implementation_name(list_type: &ImportListType) -> &'static str {
    match list_type {
        ImportListType::SonarrImport => "SonarrImport",
        ImportListType::TraktListImport => "TraktListImport",
        ImportListType::TraktUserImport => "TraktUserImport",
        ImportListType::TraktPopularImport => "TraktPopularImport",
        ImportListType::PlexImport => "PlexImport",
        ImportListType::ImdbListImport => "ImdbListImport",
        ImportListType::CustomImport => "CustomImport",
        ImportListType::SimklImport => "SimklImport",
        ImportListType::AniListImport => "AniListImport",
        ImportListType::MyAnimeListImport => "MyAnimeListImport",
    }
}

fn convert_monitor_type(mt: &MonitorTypes) -> sonarr::models::MonitorTypes {
    match mt {
        MonitorTypes::All => sonarr::models::MonitorTypes::All,
        MonitorTypes::Future => sonarr::models::MonitorTypes::Future,
        MonitorTypes::Missing => sonarr::models::MonitorTypes::Missing,
        MonitorTypes::Existing => sonarr::models::MonitorTypes::Existing,
        MonitorTypes::FirstSeason => sonarr::models::MonitorTypes::FirstSeason,
        MonitorTypes::LatestSeason => sonarr::models::MonitorTypes::LatestSeason,
        MonitorTypes::Pilot => sonarr::models::MonitorTypes::Pilot,
        MonitorTypes::MonitorSpecials => sonarr::models::MonitorTypes::MonitorSpecials,
        MonitorTypes::UnmonitorSpecials => sonarr::models::MonitorTypes::UnmonitorSpecials,
        MonitorTypes::None => sonarr::models::MonitorTypes::None,
    }
}

fn convert_new_item_monitor_type(mt: &NewItemMonitorTypes) -> sonarr::models::NewItemMonitorTypes {
    match mt {
        NewItemMonitorTypes::All => sonarr::models::NewItemMonitorTypes::All,
        NewItemMonitorTypes::None => sonarr::models::NewItemMonitorTypes::None,
    }
}

fn convert_series_type(st: &SeriesTypes) -> sonarr::models::SeriesTypes {
    match st {
        SeriesTypes::Standard => sonarr::models::SeriesTypes::Standard,
        SeriesTypes::Daily => sonarr::models::SeriesTypes::Daily,
        SeriesTypes::Anime => sonarr::models::SeriesTypes::Anime,
    }
}

async fn reconcile_cleanup(
    import_list: Arc<SonarrImportList>,
    ctx: Arc<Context>,
) -> Result<Action> {
    let client = &ctx.client;
    let namespace = import_list
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;

    info!(
        "Cleaning up SonarrImportList: {}/{}",
        namespace,
        import_list.name_any()
    );

    if let Some(id) = import_list.status.as_ref().and_then(|s| s.id)
        && let Ok(config) = get_sonarr_config(
            &ctx,
            client,
            &namespace,
            &import_list.spec.sonarr_instance_ref,
        )
        .await
    {
        let _ = import_list_api::delete_import_list(&config, id).await;
    }

    Ok(Action::await_change())
}
