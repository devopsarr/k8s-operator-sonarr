use std::sync::Arc;

use k8s_openapi::api::core::v1::Secret;
use kube::api::{Api, Patch, PatchParams};
use kube::runtime::controller::Action;
use kube::{Client, ResourceExt};
use tracing::info;

use sonarr::apis::configuration::Configuration;
use sonarr::apis::tag_api;
use sonarr::models::TagResource;

use crate::Context;
use crate::crds::{Sonarr, SonarrInstanceRef, SonarrTag, SonarrTagStatus};
use crate::error::{Error, Result};

use super::traits::{REQUEUE_DURATION, reconcile_with_finalizer, run_controller};
use super::{ready_condition, update_conditions};

/// Start the SonarrTag controller
pub async fn run(client: Client, context: Arc<Context>) {
    run_controller::<SonarrTag, _, _>(client, context, "SonarrTag", reconcile).await;
}

async fn reconcile(obj: Arc<SonarrTag>, ctx: Arc<Context>) -> Result<Action> {
    reconcile_with_finalizer(obj, ctx, reconcile_apply, reconcile_cleanup).await
}

async fn reconcile_apply(tag: Arc<SonarrTag>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = tag
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;
    let name = tag.name_any();

    info!("Reconciling SonarrTag: {}/{}", namespace, name);

    // Get the Sonarr configuration
    let config = get_sonarr_config(&ctx, client, &namespace, &tag.spec.sonarr_instance_ref).await?;

    // Get or create the tag in Sonarr
    let sonarr_tag = if let Some(id) = tag.status.as_ref().and_then(|s| s.id) {
        // Update existing tag
        let mut tag_resource = TagResource::new();
        tag_resource.id = Some(id);
        tag_resource.label = Some(Some(tag.spec.label.clone()));

        match tag_api::update_tag(&config, &id.to_string(), Some(tag_resource.clone())).await {
            Ok(t) => t,
            Err(_) => {
                // Tag might have been deleted, create a new one
                let mut new_tag = TagResource::new();
                new_tag.label = Some(Some(tag.spec.label.clone()));
                tag_api::create_tag(&config, Some(new_tag)).await?
            }
        }
    } else {
        // Check if tag already exists
        let existing_tags = tag_api::list_tag(&config).await?;
        if let Some(existing) = existing_tags
            .iter()
            .find(|t| t.label.as_ref().and_then(|l| l.as_ref()) == Some(&tag.spec.label))
        {
            existing.clone()
        } else {
            let mut new_tag = TagResource::new();
            new_tag.label = Some(Some(tag.spec.label.clone()));
            tag_api::create_tag(&config, Some(new_tag)).await?
        }
    };

    // Update status
    let tags_api: Api<SonarrTag> = Api::namespaced(client.clone(), &namespace);
    let mut conditions = tag
        .status
        .as_ref()
        .map(|s| s.conditions.clone())
        .unwrap_or_default();
    update_conditions(
        &mut conditions,
        ready_condition(true, "Synced", "Tag synchronized with Sonarr"),
    );

    let status = SonarrTagStatus {
        conditions,
        id: sonarr_tag.id,
        observed_generation: tag.metadata.generation.unwrap_or(0),
    };

    let status_patch = serde_json::json!({ "status": status });
    tags_api
        .patch_status(&name, &PatchParams::default(), &Patch::Merge(&status_patch))
        .await?;

    Ok(Action::requeue(REQUEUE_DURATION))
}

async fn reconcile_cleanup(tag: Arc<SonarrTag>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = tag
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;

    info!("Cleaning up SonarrTag: {}/{}", namespace, tag.name_any());

    if let Some(id) = tag.status.as_ref().and_then(|s| s.id)
        && let Ok(config) =
            get_sonarr_config(&ctx, client, &namespace, &tag.spec.sonarr_instance_ref).await
    {
        let _ = tag_api::delete_tag(&config, id).await;
    }

    Ok(Action::await_change())
}

pub async fn get_sonarr_config(
    ctx: &Context,
    client: &Client,
    namespace: &str,
    instance_ref: &SonarrInstanceRef,
) -> Result<Arc<Configuration>> {
    let instance_namespace = instance_ref.namespace.as_deref().unwrap_or(namespace);
    let instances: Api<Sonarr> = Api::namespaced(client.clone(), instance_namespace);

    let instance = instances
        .get(&instance_ref.name)
        .await
        .map_err(|_| Error::SonarrInstanceNotFound(instance_ref.name.clone()))?;

    // Check if instance is ready
    let is_ready = instance
        .status
        .as_ref()
        .map(|s| {
            s.conditions
                .iter()
                .any(|c| c.type_ == "Ready" && c.status == "True")
        })
        .unwrap_or(false);

    if !is_ready {
        return Err(Error::SonarrInstanceNotReady(instance_ref.name.clone()));
    }

    // Get URL
    let url = instance
        .status
        .as_ref()
        .and_then(|s| s.url.clone())
        .ok_or(Error::SonarrInstanceNotReady(instance_ref.name.clone()))?;

    // Get API key
    let api_key = get_api_key(client, instance_namespace, &instance).await?;

    let instance_key = format!("{}/{}", instance_namespace, instance_ref.name);
    Ok(ctx
        .sonarr_client_factory
        .get_config(&url, &api_key, &instance_key)
        .await)
}

pub async fn get_api_key(client: &Client, namespace: &str, instance: &Sonarr) -> Result<String> {
    let secret_name = if let Some(ref secret_ref) = instance.spec.api_key_secret_ref {
        secret_ref.name.clone()
    } else {
        instance.api_key_secret_name()
    };

    let secret_key = instance
        .spec
        .api_key_secret_ref
        .as_ref()
        .map(|s| s.key.clone())
        .unwrap_or_else(|| "api-key".to_string());

    let secrets: Api<Secret> = Api::namespaced(client.clone(), namespace);
    let secret = secrets
        .get(&secret_name)
        .await
        .map_err(|_| Error::MissingApiCredentials)?;

    let data = secret.data.ok_or(Error::MissingApiCredentials)?;
    let api_key_bytes = data.get(&secret_key).ok_or(Error::MissingApiCredentials)?;

    String::from_utf8(api_key_bytes.0.clone()).map_err(|_| Error::MissingApiCredentials)
}
