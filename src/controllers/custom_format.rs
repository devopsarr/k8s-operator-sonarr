use std::sync::Arc;

use kube::api::{Api, Patch, PatchParams};
use kube::runtime::controller::Action;
use kube::{Client, ResourceExt};
use tracing::info;

use sonarr::apis::custom_format_api;
use sonarr::models::{CustomFormatResource, CustomFormatSpecificationSchema};

use crate::Context;
use crate::crds::custom_format::{CustomFormatImplementation, CustomFormatSpecification};
use crate::crds::{SonarrCustomFormat, SonarrCustomFormatStatus};
use crate::error::{Error, Result};

use super::tag::get_sonarr_config;
use super::traits::{REQUEUE_DURATION, reconcile_with_finalizer, run_controller};
use super::{ready_condition, update_conditions};

/// Start the SonarrCustomFormat controller
pub async fn run(client: Client, context: Arc<Context>) {
    run_controller::<SonarrCustomFormat, _, _>(client, context, "SonarrCustomFormat", reconcile)
        .await;
}

async fn reconcile(obj: Arc<SonarrCustomFormat>, ctx: Arc<Context>) -> Result<Action> {
    reconcile_with_finalizer(obj, ctx, reconcile_apply, reconcile_cleanup).await
}

async fn reconcile_apply(cf: Arc<SonarrCustomFormat>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = cf
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;
    let name = cf.name_any();

    info!("Reconciling SonarrCustomFormat: {}/{}", namespace, name);

    let config = get_sonarr_config(&ctx, client, &namespace, &cf.spec.sonarr_instance_ref).await?;

    // Build custom format resource
    let mut resource = CustomFormatResource::new();
    resource.name = Some(Some(cf.spec.name.clone()));
    resource.include_custom_format_when_renaming =
        Some(Some(cf.spec.include_custom_format_when_renaming));

    // Convert specifications
    let specs: Vec<CustomFormatSpecificationSchema> = cf
        .spec
        .specifications
        .iter()
        .map(|spec| convert_specification(spec))
        .collect();
    resource.specifications = Some(Some(specs));

    let sonarr_cf = if let Some(id) = cf.status.as_ref().and_then(|s| s.id) {
        resource.id = Some(id);
        match custom_format_api::update_custom_format(
            &config,
            &id.to_string(),
            Some(resource.clone()),
        )
        .await
        {
            Ok(c) => c,
            Err(_) => {
                resource.id = None;
                custom_format_api::create_custom_format(&config, Some(resource)).await?
            }
        }
    } else {
        let existing = custom_format_api::list_custom_format(&config).await?;
        if let Some(existing_item) = existing
            .iter()
            .find(|c| c.name.as_ref().and_then(|n| n.as_ref()) == Some(&cf.spec.name))
        {
            existing_item.clone()
        } else {
            custom_format_api::create_custom_format(&config, Some(resource)).await?
        }
    };

    // Update status
    let api: Api<SonarrCustomFormat> = Api::namespaced(client.clone(), &namespace);
    let mut conditions = cf
        .status
        .as_ref()
        .map(|s| s.conditions.clone())
        .unwrap_or_default();
    update_conditions(
        &mut conditions,
        ready_condition(true, "Synced", "Custom format synchronized with Sonarr"),
    );

    let status = SonarrCustomFormatStatus {
        conditions,
        id: sonarr_cf.id,
        observed_generation: cf.metadata.generation.unwrap_or(0),
    };

    let status_patch = serde_json::json!({ "status": status });
    api.patch_status(&name, &PatchParams::default(), &Patch::Merge(&status_patch))
        .await?;

    Ok(Action::requeue(REQUEUE_DURATION))
}

fn convert_specification(spec: &CustomFormatSpecification) -> CustomFormatSpecificationSchema {
    let mut schema = CustomFormatSpecificationSchema::new();
    schema.name = Some(Some(spec.name.clone()));
    schema.implementation = Some(Some(
        get_implementation_name(&spec.implementation).to_string(),
    ));
    schema.negate = Some(spec.negate);
    schema.required = Some(spec.required);
    // Fields would need more complex mapping
    schema
}

fn get_implementation_name(impl_type: &CustomFormatImplementation) -> &'static str {
    match impl_type {
        CustomFormatImplementation::ReleaseTitleSpecification => "ReleaseTitleSpecification",
        CustomFormatImplementation::SourceSpecification => "SourceSpecification",
        CustomFormatImplementation::ResolutionSpecification => "ResolutionSpecification",
        CustomFormatImplementation::QualityModifierSpecification => "QualityModifierSpecification",
        CustomFormatImplementation::SizeSpecification => "SizeSpecification",
        CustomFormatImplementation::IndexerFlagSpecification => "IndexerFlagSpecification",
        CustomFormatImplementation::LanguageSpecification => "LanguageSpecification",
        CustomFormatImplementation::ReleaseGroupSpecification => "ReleaseGroupSpecification",
        CustomFormatImplementation::EditionSpecification => "EditionSpecification",
    }
}

async fn reconcile_cleanup(cf: Arc<SonarrCustomFormat>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = cf
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;

    info!(
        "Cleaning up SonarrCustomFormat: {}/{}",
        namespace,
        cf.name_any()
    );

    if let Some(id) = cf.status.as_ref().and_then(|s| s.id) {
        if let Ok(config) =
            get_sonarr_config(&ctx, client, &namespace, &cf.spec.sonarr_instance_ref).await
        {
            let _ = custom_format_api::delete_custom_format(&config, id).await;
        }
    }

    Ok(Action::await_change())
}
