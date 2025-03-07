use crate::crd::sonarr::Sonarr;
use crate::error::Error;
use chrono::Utc;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Time;
use kube::{
    api::{Api, Patch, PatchParams},
    client::Client,
    runtime::controller::Action,
    ResourceExt,
};
use serde_json::json;
use std::{sync::Arc, time::Duration};
use tracing::{error, info, warn};

/// State shared by the controller
pub struct SonarrState {
    /// Kubernetes client
    pub client: Client,
}

impl SonarrState {
    pub fn new(client: Client) -> Self {
        SonarrState { client }
    }
}

/// Main reconciliation loop for the controller
pub async fn reconcile(
    resource: Arc<Sonarr>,
    context: Arc<SonarrState>,
) -> Result<Action, Error> {
    let client = context.client.clone();
    let name = resource.name_any();
    let namespace = resource.namespace().unwrap_or_else(|| "default".into());
    
    info!("Reconciling Sonarr {} in {}", name, namespace);

    // Get API for Sonnar in the appropriate namespace
    let api: Api<Sonarr> = Api::namespaced(client, &namespace);

    // Initialize or update the status
    let current_time = Utc::now();
    let status_patch = json!({
        "status": {
            "observedGeneration": resource.metadata.generation,
            "createdAt": current_time,
            "conditions": [{
                "type": "Initialized",
                "status": "True",
                "lastTransitionTime": Time(current_time),
                "reason": "ResourceCreated",
                "message": "Resource has been initialized"
            }],
            "ready": true
        }
    });

    // Apply the status update
    let patch_params = PatchParams::apply("k8s-operator-sonarr").force();
    let _patched = api
        .patch_status(&name, &patch_params, &Patch::Merge(&status_patch))
        .await
        .map_err(|err| {
            error!("Failed to patch status: {}", err);
            Error::KubeError(err)
        })?;

    info!("Successfully reconciled Sonnar {} in {}", name, namespace);
    
    // Requeue after 10 minutes by default
    Ok(Action::requeue(Duration::from_secs(600)))
}

/// Error policy for the controller
pub fn error_policy(resource: Arc<Sonarr>, error: &Error, _ctx: Arc<SonarrState>) -> Action {
    warn!("Reconciliation error: {:?}", error);
    // Retry after 5 minutes
    Action::requeue(Duration::from_secs(300))
}