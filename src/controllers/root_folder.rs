use std::sync::Arc;

use kube::api::{Api, Patch, PatchParams};
use kube::runtime::controller::Action;
use kube::{Client, ResourceExt};
use tracing::info;

use sonarr::apis::root_folder_api;
use sonarr::models::RootFolderResource;

use crate::Context;
use crate::crds::{SonarrRootFolder, SonarrRootFolderStatus};
use crate::error::{Error, Result};

use super::tag::get_sonarr_config;
use super::traits::{REQUEUE_DURATION, reconcile_with_finalizer, run_controller};
use super::{ready_condition, update_conditions};

/// Start the SonarrRootFolder controller
pub async fn run(client: Client, context: Arc<Context>) {
    run_controller::<SonarrRootFolder, _, _>(client, context, "SonarrRootFolder", reconcile).await;
}

async fn reconcile(obj: Arc<SonarrRootFolder>, ctx: Arc<Context>) -> Result<Action> {
    reconcile_with_finalizer(obj, ctx, reconcile_apply, reconcile_cleanup).await
}

async fn reconcile_apply(folder: Arc<SonarrRootFolder>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = folder
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;
    let name = folder.name_any();

    info!("Reconciling SonarrRootFolder: {}/{}", namespace, name);

    let config =
        get_sonarr_config(&ctx, client, &namespace, &folder.spec.sonarr_instance_ref).await?;

    // Get or create root folder
    let sonarr_folder = if let Some(id) = folder.status.as_ref().and_then(|s| s.id) {
        match root_folder_api::get_root_folder_by_id(&config, id).await {
            Ok(f) => f,
            Err(_) => {
                let mut rf = RootFolderResource::new();
                rf.path = Some(Some(folder.spec.path.clone()));
                root_folder_api::create_root_folder(&config, Some(rf)).await?
            }
        }
    } else {
        let existing = root_folder_api::list_root_folder(&config).await?;
        if let Some(existing_folder) = existing
            .iter()
            .find(|f| f.path.as_ref().and_then(|p| p.as_ref()) == Some(&folder.spec.path))
        {
            existing_folder.clone()
        } else {
            let mut rf = RootFolderResource::new();
            rf.path = Some(Some(folder.spec.path.clone()));
            root_folder_api::create_root_folder(&config, Some(rf)).await?
        }
    };

    // Update status
    let folders_api: Api<SonarrRootFolder> = Api::namespaced(client.clone(), &namespace);
    let mut conditions = folder
        .status
        .as_ref()
        .map(|s| s.conditions.clone())
        .unwrap_or_default();
    update_conditions(
        &mut conditions,
        ready_condition(true, "Synced", "Root folder synchronized with Sonarr"),
    );

    let status = SonarrRootFolderStatus {
        conditions,
        id: sonarr_folder.id,
        accessible: sonarr_folder.accessible,
        free_space: sonarr_folder.free_space.flatten(),
        observed_generation: folder.metadata.generation.unwrap_or(0),
    };

    let status_patch = serde_json::json!({ "status": status });
    folders_api
        .patch_status(&name, &PatchParams::default(), &Patch::Merge(&status_patch))
        .await?;

    Ok(Action::requeue(REQUEUE_DURATION))
}

async fn reconcile_cleanup(folder: Arc<SonarrRootFolder>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = folder
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;

    info!(
        "Cleaning up SonarrRootFolder: {}/{}",
        namespace,
        folder.name_any()
    );

    if let Some(id) = folder.status.as_ref().and_then(|s| s.id) {
        if let Ok(config) =
            get_sonarr_config(&ctx, client, &namespace, &folder.spec.sonarr_instance_ref).await
        {
            let _ = root_folder_api::delete_root_folder(&config, id).await;
        }
    }

    Ok(Action::await_change())
}
