use std::sync::Arc;

use kube::api::{Api, Patch, PatchParams};
use kube::runtime::controller::Action;
use kube::{Client, ResourceExt};
use tracing::info;

use sonarr::apis::quality_definition_api;
use sonarr::models::QualityDefinitionResource;

use crate::Context;
use crate::crds::{SonarrQualityDefinition, SonarrQualityDefinitionStatus};
use crate::error::{Error, Result};

use super::tag::get_sonarr_config;
use super::traits::{REQUEUE_DURATION, reconcile_with_finalizer, run_controller};
use super::{ready_condition, update_conditions};

/// Start the SonarrQualityDefinition controller
pub async fn run(client: Client, context: Arc<Context>) {
    run_controller::<SonarrQualityDefinition, _, _>(
        client,
        context,
        "SonarrQualityDefinition",
        reconcile,
    )
    .await;
}

async fn reconcile(obj: Arc<SonarrQualityDefinition>, ctx: Arc<Context>) -> Result<Action> {
    reconcile_with_finalizer(obj, ctx, reconcile_apply, reconcile_cleanup).await
}

async fn reconcile_apply(qd: Arc<SonarrQualityDefinition>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = qd
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;
    let name = qd.name_any();

    info!(
        "Reconciling SonarrQualityDefinition: {}/{}",
        namespace, name
    );

    let config = get_sonarr_config(&ctx, client, &namespace, &qd.spec.sonarr_instance_ref).await?;

    // Quality definitions already exist in Sonarr, we just update them
    let quality_id = qd.spec.quality_name.to_quality_id();

    // Get existing quality definitions to find the one we want to update
    let existing = quality_definition_api::list_quality_definition(&config).await?;
    let existing_qd = existing
        .iter()
        .find(|q| q.quality.as_ref().and_then(|quality| quality.id) == Some(quality_id));

    let sonarr_qd = if let Some(existing_item) = existing_qd {
        let mut resource = QualityDefinitionResource::new();
        resource.id = existing_item.id;
        resource.quality = existing_item.quality.clone();
        resource.weight = existing_item.weight;

        // Apply our customizations
        if let Some(title) = &qd.spec.title {
            resource.title = Some(Some(title.clone()));
        } else {
            resource.title = existing_item.title.clone();
        }
        resource.min_size = Some(qd.spec.min_size);
        resource.max_size = Some(qd.spec.max_size);
        resource.preferred_size = Some(qd.spec.preferred_size);

        let id = existing_item
            .id
            .ok_or(Error::MissingObjectKey("quality_definition.id"))?;
        quality_definition_api::update_quality_definition(&config, &id.to_string(), Some(resource))
            .await?
    } else {
        return Err(Error::SonarrApiError(format!(
            "Quality definition for {:?} not found in Sonarr",
            qd.spec.quality_name
        )));
    };

    // Update status
    let api: Api<SonarrQualityDefinition> = Api::namespaced(client.clone(), &namespace);
    let mut conditions = qd
        .status
        .as_ref()
        .map(|s| s.conditions.clone())
        .unwrap_or_default();
    update_conditions(
        &mut conditions,
        ready_condition(
            true,
            "Synced",
            "Quality definition synchronized with Sonarr",
        ),
    );

    let status = SonarrQualityDefinitionStatus {
        conditions,
        id: sonarr_qd.id,
        observed_generation: qd.metadata.generation.unwrap_or(0),
    };

    let status_patch = serde_json::json!({ "status": status });
    api.patch_status(&name, &PatchParams::default(), &Patch::Merge(&status_patch))
        .await?;

    Ok(Action::requeue(REQUEUE_DURATION))
}

async fn reconcile_cleanup(qd: Arc<SonarrQualityDefinition>, _ctx: Arc<Context>) -> Result<Action> {
    let namespace = qd
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;

    info!(
        "Cleaning up SonarrQualityDefinition: {}/{}",
        namespace,
        qd.name_any()
    );

    // Quality definitions cannot be deleted in Sonarr, only modified
    // So cleanup is a no-op - we just let the CRD be removed
    // The quality definition in Sonarr will remain with its current settings

    Ok(Action::await_change())
}
