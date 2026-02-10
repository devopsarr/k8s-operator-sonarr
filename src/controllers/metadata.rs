use std::sync::Arc;

use kube::api::{Api, Patch, PatchParams};
use kube::runtime::controller::Action;
use kube::{Client, ResourceExt};
use tracing::info;

use sonarr::apis::metadata_api;
use sonarr::models::MetadataResource;

use crate::crds::{SonarrMetadata, SonarrMetadataStatus};
use crate::crds::metadata::MetadataType;
use crate::error::{Error, Result};
use crate::Context;

use super::traits::{run_controller, reconcile_with_finalizer, REQUEUE_DURATION};
use super::tag::get_sonarr_config;
use super::{ready_condition, update_conditions};

/// Start the SonarrMetadata controller
pub async fn run(client: Client, context: Arc<Context>) {
    run_controller::<SonarrMetadata, _, _>(
        client,
        context,
        "SonarrMetadata",
        reconcile,
    ).await;
}

async fn reconcile(obj: Arc<SonarrMetadata>, ctx: Arc<Context>) -> Result<Action> {
    reconcile_with_finalizer(obj, ctx, reconcile_apply, reconcile_cleanup).await
}

async fn reconcile_apply(metadata: Arc<SonarrMetadata>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = metadata.namespace().ok_or(Error::MissingObjectKey(".metadata.namespace"))?;
    let name = metadata.name_any();

    info!("Reconciling SonarrMetadata: {}/{}", namespace, name);

    let config = get_sonarr_config(&ctx, client, &namespace, &metadata.spec.sonarr_instance_ref).await?;

    // Build metadata resource
    let mut resource = MetadataResource::new();
    resource.name = Some(Some(metadata.spec.name.clone()));
    resource.implementation = Some(Some(get_implementation_name(&metadata.spec.metadata_type).to_string()));
    resource.config_contract = Some(Some(get_config_contract(&metadata.spec.metadata_type).to_string()));
    resource.enable = Some(metadata.spec.enable);
    resource.tags = Some(Some(metadata.spec.tags.clone()));

    // Fields would need more complex mapping for the sonarr crate

    let sonarr_metadata = if let Some(id) = metadata.status.as_ref().and_then(|s| s.id) {
        resource.id = Some(id);
        match metadata_api::update_metadata(&config, id, Some(false), Some(resource.clone())).await {
            Ok(m) => m,
            Err(_) => {
                resource.id = None;
                metadata_api::create_metadata(&config, Some(false), Some(resource)).await?
            }
        }
    } else {
        let existing = metadata_api::list_metadata(&config).await?;
        if let Some(existing_item) = existing.iter().find(|m| m.name.as_ref().and_then(|n| n.as_ref()) == Some(&metadata.spec.name)) {
            existing_item.clone()
        } else {
            metadata_api::create_metadata(&config, Some(false), Some(resource)).await?
        }
    };

    // Update status
    let api: Api<SonarrMetadata> = Api::namespaced(client.clone(), &namespace);
    let mut conditions = metadata.status.as_ref().map(|s| s.conditions.clone()).unwrap_or_default();
    update_conditions(&mut conditions, ready_condition(true, "Synced", "Metadata synchronized with Sonarr"));

    let status = SonarrMetadataStatus {
        conditions,
        id: sonarr_metadata.id,
        observed_generation: metadata.metadata.generation.unwrap_or(0),
    };

    let status_patch = serde_json::json!({ "status": status });
    api.patch_status(&name, &PatchParams::default(), &Patch::Merge(&status_patch)).await?;

    Ok(Action::requeue(REQUEUE_DURATION))
}

fn get_implementation_name(metadata_type: &MetadataType) -> &'static str {
    match metadata_type {
        MetadataType::XbmcMetadata => "XbmcMetadata",
        MetadataType::RoksboxMetadata => "RoksboxMetadata",
        MetadataType::WdtvMetadata => "WdtvMetadata",
    }
}

fn get_config_contract(metadata_type: &MetadataType) -> &'static str {
    match metadata_type {
        MetadataType::XbmcMetadata => "XbmcMetadataSettings",
        MetadataType::RoksboxMetadata => "RoksboxMetadataSettings",
        MetadataType::WdtvMetadata => "WdtvMetadataSettings",
    }
}

async fn reconcile_cleanup(metadata: Arc<SonarrMetadata>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = metadata.namespace().ok_or(Error::MissingObjectKey(".metadata.namespace"))?;

    info!("Cleaning up SonarrMetadata: {}/{}", namespace, metadata.name_any());

    if let Some(id) = metadata.status.as_ref().and_then(|s| s.id) {
        if let Ok(config) = get_sonarr_config(&ctx, client, &namespace, &metadata.spec.sonarr_instance_ref).await {
            let _ = metadata_api::delete_metadata(&config, id).await;
        }
    }

    Ok(Action::await_change())
}
