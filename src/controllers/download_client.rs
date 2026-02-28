use std::sync::Arc;

use kube::api::{Api, Patch, PatchParams};
use kube::runtime::controller::Action;
use kube::{Client, ResourceExt};
use tracing::info;

use sonarr::apis::download_client_api;
use sonarr::models::DownloadClientResource;

use crate::Context;
use crate::crds::download_client::DownloadClientType;
use crate::crds::{SonarrDownloadClient, SonarrDownloadClientStatus};
use crate::error::{Error, Result};

use super::tag::get_sonarr_config;
use super::traits::{REQUEUE_DURATION, reconcile_with_finalizer, run_controller};
use super::{ready_condition, update_conditions};

/// Start the SonarrDownloadClient controller
pub async fn run(client: Client, context: Arc<Context>) {
    run_controller::<SonarrDownloadClient, _, _>(
        client,
        context,
        "SonarrDownloadClient",
        reconcile,
    )
    .await;
}

async fn reconcile(obj: Arc<SonarrDownloadClient>, ctx: Arc<Context>) -> Result<Action> {
    reconcile_with_finalizer(obj, ctx, reconcile_apply, reconcile_cleanup).await
}

async fn reconcile_apply(dc: Arc<SonarrDownloadClient>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = dc
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;
    let name = dc.name_any();

    info!("Reconciling SonarrDownloadClient: {}/{}", namespace, name);

    let config = get_sonarr_config(&ctx, client, &namespace, &dc.spec.sonarr_instance_ref).await?;

    // Build download client resource
    let mut dc_resource = DownloadClientResource::new();
    dc_resource.name = Some(Some(dc.spec.name.clone()));
    dc_resource.implementation = Some(Some(
        get_implementation_name(&dc.spec.download_client_type).to_string(),
    ));
    dc_resource.enable = Some(dc.spec.enable);
    dc_resource.priority = Some(dc.spec.priority);
    dc_resource.remove_completed_downloads = Some(dc.spec.remove_completed_downloads);
    dc_resource.remove_failed_downloads = Some(dc.spec.remove_failed_downloads);
    dc_resource.tags = Some(Some(dc.spec.tags.clone()));

    let sonarr_dc = if let Some(id) = dc.status.as_ref().and_then(|s| s.id) {
        dc_resource.id = Some(id);
        match download_client_api::update_download_client(
            &config,
            id,
            Some(false),
            Some(dc_resource.clone()),
        )
        .await
        {
            Ok(d) => d,
            Err(_) => {
                dc_resource.id = None;
                download_client_api::create_download_client(&config, Some(false), Some(dc_resource))
                    .await?
            }
        }
    } else {
        let existing = download_client_api::list_download_client(&config).await?;
        if let Some(existing_dc) = existing
            .iter()
            .find(|d| d.name.as_ref().and_then(|n| n.as_ref()) == Some(&dc.spec.name))
        {
            existing_dc.clone()
        } else {
            download_client_api::create_download_client(&config, Some(false), Some(dc_resource))
                .await?
        }
    };

    // Update status
    let clients_api: Api<SonarrDownloadClient> = Api::namespaced(client.clone(), &namespace);
    let mut conditions = dc
        .status
        .as_ref()
        .map(|s| s.conditions.clone())
        .unwrap_or_default();
    update_conditions(
        &mut conditions,
        ready_condition(true, "Synced", "Download client synchronized with Sonarr"),
    );

    let status = SonarrDownloadClientStatus {
        conditions,
        id: sonarr_dc.id,
        observed_generation: dc.metadata.generation.unwrap_or(0),
    };

    let status_patch = serde_json::json!({ "status": status });
    clients_api
        .patch_status(&name, &PatchParams::default(), &Patch::Merge(&status_patch))
        .await?;

    Ok(Action::requeue(REQUEUE_DURATION))
}

fn get_implementation_name(dc_type: &DownloadClientType) -> &'static str {
    match dc_type {
        DownloadClientType::Aria2 => "Aria2",
        DownloadClientType::Deluge => "Deluge",
        DownloadClientType::Flood => "Flood",
        DownloadClientType::Hadouken => "Hadouken",
        DownloadClientType::NzbGet => "Nzbget",
        DownloadClientType::NzbVortex => "NzbVortex",
        DownloadClientType::Pneumatic => "Pneumatic",
        DownloadClientType::QBittorrent => "QBittorrent",
        DownloadClientType::RTorrent => "RTorrent",
        DownloadClientType::SABnzbd => "Sabnzbd",
        DownloadClientType::TorrentBlackhole => "TorrentBlackhole",
        DownloadClientType::TorrentDownloadStation => "TorrentDownloadStation",
        DownloadClientType::Transmission => "Transmission",
        DownloadClientType::UsenetBlackhole => "UsenetBlackhole",
        DownloadClientType::UsenetDownloadStation => "UsenetDownloadStation",
        DownloadClientType::UTorrent => "UTorrent",
        DownloadClientType::Vuze => "Vuze",
    }
}

async fn reconcile_cleanup(dc: Arc<SonarrDownloadClient>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = dc
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;

    info!(
        "Cleaning up SonarrDownloadClient: {}/{}",
        namespace,
        dc.name_any()
    );

    if let Some(id) = dc.status.as_ref().and_then(|s| s.id)
        && let Ok(config) =
            get_sonarr_config(&ctx, client, &namespace, &dc.spec.sonarr_instance_ref).await
    {
        let _ = download_client_api::delete_download_client(&config, id).await;
    }

    Ok(Action::await_change())
}
