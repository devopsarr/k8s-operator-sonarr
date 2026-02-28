use std::sync::Arc;

use kube::api::{Api, Patch, PatchParams};
use kube::runtime::controller::Action;
use kube::{Client, ResourceExt};
use tracing::info;

use sonarr::apis::auto_tagging_api;
use sonarr::models::{AutoTaggingResource, AutoTaggingSpecificationSchema, Field};

use crate::Context;
use crate::crds::auto_tag::{AutoTagImplementation, AutoTagSpecification};
use crate::crds::{SonarrAutoTag, SonarrAutoTagStatus};
use crate::error::{Error, Result};

use super::tag::get_sonarr_config;
use super::traits::{REQUEUE_DURATION, reconcile_with_finalizer, run_controller};
use super::{ready_condition, update_conditions};

/// Start the SonarrAutoTag controller
pub async fn run(client: Client, context: Arc<Context>) {
    run_controller::<SonarrAutoTag, _, _>(client, context, "SonarrAutoTag", reconcile).await;
}

async fn reconcile(obj: Arc<SonarrAutoTag>, ctx: Arc<Context>) -> Result<Action> {
    reconcile_with_finalizer(obj, ctx, reconcile_apply, reconcile_cleanup).await
}

async fn reconcile_apply(auto_tag: Arc<SonarrAutoTag>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = auto_tag
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;
    let name = auto_tag.name_any();

    info!("Reconciling SonarrAutoTag: {}/{}", namespace, name);

    let config =
        get_sonarr_config(&ctx, client, &namespace, &auto_tag.spec.sonarr_instance_ref).await?;

    // Build auto-tagging resource
    let mut resource = AutoTaggingResource::new();
    resource.name = Some(Some(auto_tag.spec.name.clone()));
    resource.remove_tags_automatically = Some(auto_tag.spec.remove_tags_automatically);
    resource.tags = Some(Some(auto_tag.spec.tags.clone()));

    // Convert specifications
    let specs: Vec<AutoTaggingSpecificationSchema> = auto_tag
        .spec
        .specifications
        .iter()
        .map(convert_specification)
        .collect();
    resource.specifications = Some(Some(specs));

    let sonarr_auto_tag = if let Some(id) = auto_tag.status.as_ref().and_then(|s| s.id) {
        resource.id = Some(id);
        match auto_tagging_api::update_auto_tagging(
            &config,
            &id.to_string(),
            Some(resource.clone()),
        )
        .await
        {
            Ok(a) => a,
            Err(_) => {
                resource.id = None;
                auto_tagging_api::create_auto_tagging(&config, Some(resource)).await?
            }
        }
    } else {
        let existing = auto_tagging_api::list_auto_tagging(&config).await?;
        if let Some(existing_item) = existing
            .iter()
            .find(|a| a.name.as_ref().and_then(|n| n.as_ref()) == Some(&auto_tag.spec.name))
        {
            existing_item.clone()
        } else {
            auto_tagging_api::create_auto_tagging(&config, Some(resource)).await?
        }
    };

    // Update status
    let api: Api<SonarrAutoTag> = Api::namespaced(client.clone(), &namespace);
    let mut conditions = auto_tag
        .status
        .as_ref()
        .map(|s| s.conditions.clone())
        .unwrap_or_default();
    update_conditions(
        &mut conditions,
        ready_condition(true, "Synced", "Auto tag synchronized with Sonarr"),
    );

    let status = SonarrAutoTagStatus {
        conditions,
        id: sonarr_auto_tag.id,
        observed_generation: auto_tag.metadata.generation.unwrap_or(0),
    };

    let status_patch = serde_json::json!({ "status": status });
    api.patch_status(&name, &PatchParams::default(), &Patch::Merge(&status_patch))
        .await?;

    Ok(Action::requeue(REQUEUE_DURATION))
}

fn convert_specification(spec: &AutoTagSpecification) -> AutoTaggingSpecificationSchema {
    let mut schema = AutoTaggingSpecificationSchema::new();
    schema.name = Some(Some(spec.name.clone()));
    schema.implementation = Some(Some(
        get_implementation_name(&spec.implementation).to_string(),
    ));
    schema.negate = Some(spec.negate);
    schema.required = Some(spec.required);

    // Map fields to Sonarr API Field objects
    let mut fields = Vec::new();
    if let Some(ref value) = spec.fields.value {
        let mut field = Field::new();
        field.name = Some(Some("value".to_string()));
        field.value = Some(Some(serde_json::Value::String(value.clone())));
        fields.push(field);
    }
    if let Some(min) = spec.fields.min {
        let mut field = Field::new();
        field.name = Some(Some("min".to_string()));
        field.value = Some(Some(serde_json::json!(min)));
        fields.push(field);
    }
    if let Some(max) = spec.fields.max {
        let mut field = Field::new();
        field.name = Some(Some("max".to_string()));
        field.value = Some(Some(serde_json::json!(max)));
        fields.push(field);
    }
    if !fields.is_empty() {
        schema.fields = Some(Some(fields));
    }

    schema
}

fn get_implementation_name(impl_type: &AutoTagImplementation) -> &'static str {
    match impl_type {
        AutoTagImplementation::RootFolderSpecification => "RootFolderSpecification",
        AutoTagImplementation::GenreSpecification => "GenreSpecification",
        AutoTagImplementation::YearSpecification => "YearSpecification",
        AutoTagImplementation::SeriesTypeSpecification => "SeriesTypeSpecification",
        AutoTagImplementation::QualityProfileSpecification => "QualityProfileSpecification",
        AutoTagImplementation::NetworkSpecification => "NetworkSpecification",
        AutoTagImplementation::OriginalLanguageSpecification => "OriginalLanguageSpecification",
        AutoTagImplementation::TagSpecification => "TagSpecification",
    }
}

async fn reconcile_cleanup(auto_tag: Arc<SonarrAutoTag>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = auto_tag
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;

    info!(
        "Cleaning up SonarrAutoTag: {}/{}",
        namespace,
        auto_tag.name_any()
    );

    if let Some(id) = auto_tag.status.as_ref().and_then(|s| s.id)
        && let Ok(config) =
            get_sonarr_config(&ctx, client, &namespace, &auto_tag.spec.sonarr_instance_ref).await
    {
        let _ = auto_tagging_api::delete_auto_tagging(&config, id).await;
    }

    Ok(Action::await_change())
}
