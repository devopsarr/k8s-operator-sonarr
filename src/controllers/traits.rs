//! Common traits and utilities for Sonarr sub-resource controllers
//!
//! This module provides:
//! - Generic controller run function with built-in finalizer support
//! - Generic error policy
//! - Common status update functions
//! - Shared utilities to reduce boilerplate across controllers

use std::fmt::Debug;
use std::future::Future;
use std::hash::Hash;
use std::sync::Arc;
use std::time::Duration;

use futures::StreamExt;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use k8s_openapi::NamespaceResourceScope;
use kube::api::{Api, Patch, PatchParams};
use kube::runtime::controller::{Action, Controller};
use kube::runtime::finalizer::{finalizer, Event};
use kube::runtime::watcher;
use kube::{Client, Resource, ResourceExt};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;
use tracing::{debug, error, info};

use crate::crds::{SonarrInstanceRef, FINALIZER};
use crate::error::{Error, Result};
use crate::Context;

use super::{ready_condition, update_conditions};

// Re-export the get_sonarr_config from tag module as the canonical implementation
pub use super::tag::get_sonarr_config;

/// Trait for resource specs that reference a Sonarr instance
pub trait HasSonarrInstanceRef {
    fn sonarr_instance_ref(&self) -> &SonarrInstanceRef;
}

/// Generic error policy for all controllers
pub fn error_policy<R>(resource_name: &'static str) -> impl Fn(Arc<R>, &Error, Arc<Context>) -> Action
where
    R: Resource + ResourceExt,
{
    move |obj: Arc<R>, err: &Error, _ctx: Arc<Context>| {
        error!("Error reconciling {} {}: {:?}", resource_name, obj.name_any(), err);
        Action::requeue(Duration::from_secs(60))
    }
}

/// Wrapper that handles the common finalizer pattern for sub-resources
/// This eliminates boilerplate from individual controllers by wrapping
/// the kube-rs finalizer function with our standard error handling
pub async fn reconcile_with_finalizer<R, ApplyFn, ApplyFut, CleanupFn, CleanupFut>(
    obj: Arc<R>,
    ctx: Arc<Context>,
    apply_fn: ApplyFn,
    cleanup_fn: CleanupFn,
) -> Result<Action>
where
    R: Resource<Scope = NamespaceResourceScope, DynamicType = ()>
        + Clone
        + Debug
        + DeserializeOwned
        + Serialize
        + Send
        + Sync
        + 'static,
    ApplyFn: FnOnce(Arc<R>, Arc<Context>) -> ApplyFut,
    ApplyFut: Future<Output = Result<Action>> + Send,
    CleanupFn: FnOnce(Arc<R>, Arc<Context>) -> CleanupFut,
    CleanupFut: Future<Output = Result<Action>> + Send,
{
    let client = &ctx.client;
    let namespace = obj
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;

    let api: Api<R> = Api::namespaced(client.clone(), &namespace);

    finalizer(&api, FINALIZER, obj.clone(), |event| async {
        match event {
            Event::Apply(resource) => apply_fn(resource, ctx.clone()).await,
            Event::Cleanup(resource) => cleanup_fn(resource, ctx.clone()).await,
        }
    })
    .await
    .map_err(|e| Error::FinalizerError(Box::new(e)))
}

/// Start a generic controller for a Sonarr sub-resource
pub async fn run_controller<R, ReconcileFn, ReconcileFut>(
    client: Client,
    context: Arc<Context>,
    resource_name: &'static str,
    reconcile_fn: ReconcileFn,
) where
    R: Resource<DynamicType = ()>
        + Clone
        + Debug
        + DeserializeOwned
        + Send
        + Sync
        + 'static,
    R::DynamicType: Default + Eq + Hash + Clone,
    ReconcileFn: FnMut(Arc<R>, Arc<Context>) -> ReconcileFut + Send + Sync + 'static + Clone,
    ReconcileFut: Future<Output = Result<Action>> + Send + 'static,
{
    let resources = Api::<R>::all(client.clone());

    info!("Starting {} controller", resource_name);

    let error_handler = error_policy::<R>(resource_name);

    Controller::new(resources, watcher::Config::default())
        .shutdown_on_signal()
        .run(reconcile_fn, error_handler, context)
        .for_each(|res| async move {
            match res {
                Ok(o) => debug!("Reconciled {}: {:?}", resource_name, o),
                Err(e) => error!("Reconcile error: {:?}", e),
            }
        })
        .await;
}

/// Update the status of a Sonarr resource with success
pub async fn update_status_success<R>(
    client: &Client,
    namespace: &str,
    name: &str,
    sonarr_id: i32,
    generation: i64,
    existing_conditions: Vec<Condition>,
) -> Result<()>
where
    R: Resource<Scope = k8s_openapi::NamespaceResourceScope> + Clone + Debug + DeserializeOwned + Serialize,
    R: Resource<DynamicType = ()>,
{
    let api: Api<R> = Api::namespaced(client.clone(), namespace);
    let mut conditions = existing_conditions;
    update_conditions(
        &mut conditions,
        ready_condition(true, "Synced", "Resource synced with Sonarr"),
    );

    let status = json!({
        "status": {
            "conditions": conditions,
            "id": sonarr_id,
            "observedGeneration": generation
        }
    });

    api.patch_status(name, &PatchParams::apply("sonarr-operator"), &Patch::Merge(&status))
        .await
        .map_err(|e| Error::KubeError(e))?;

    info!("Updated {} {}/{} status with id={}", std::any::type_name::<R>(), namespace, name, sonarr_id);
    Ok(())
}

/// Update the status of a Sonarr resource with failure
pub async fn update_status_failure<R>(
    client: &Client,
    namespace: &str,
    name: &str,
    error_message: &str,
    existing_conditions: Vec<Condition>,
) -> Result<()>
where
    R: Resource<Scope = k8s_openapi::NamespaceResourceScope> + Clone + Debug + DeserializeOwned + Serialize,
    R: Resource<DynamicType = ()>,
{
    let api: Api<R> = Api::namespaced(client.clone(), namespace);
    let mut conditions = existing_conditions;
    update_conditions(
        &mut conditions,
        ready_condition(false, "Error", error_message),
    );

    let status = json!({
        "status": {
            "conditions": conditions
        }
    });

    api.patch_status(name, &PatchParams::apply("sonarr-operator"), &Patch::Merge(&status))
        .await
        .map_err(|e| Error::KubeError(e))?;

    Ok(())
}

/// Common requeue duration for successful reconciliation
pub const REQUEUE_DURATION: Duration = Duration::from_secs(300);

/// Common requeue duration for errors
pub const ERROR_REQUEUE_DURATION: Duration = Duration::from_secs(60);
