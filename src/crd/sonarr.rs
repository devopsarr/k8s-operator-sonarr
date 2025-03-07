use chrono::{DateTime, Utc};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Time;
use kube::CustomResource;
use schemars::JsonSchema; 
use serde::{Deserialize, Serialize};

/// Define the Custom Resource Definition (CRD) for our operator
#[derive(CustomResource, Serialize, Deserialize, Default, Debug, Clone, JsonSchema)]
#[kube(
    group = "devopsarr.io",
    version = "v1",
    kind = "Sonarr",
    plural = "sonarrs",
    shortname = "son",
    namespaced
)]
#[kube(status = "SonarrStatus")]
pub struct SonarrSpec {
    pub message: String,
    pub replica_count: Option<i32>,
}

/// Status object for our CRD
#[derive(Serialize, Deserialize, Default, Debug, Clone, JsonSchema)]
pub struct SonarrStatus {
   // pub created_at: Option<DateTime<Utc>>,
    pub conditions: Vec<SonarrCondition>,
    pub observed_generation: Option<i64>,
    pub ready: Option<bool>,
}

/// Condition type for tracking the status of the resource
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SonarrCondition {
    #[serde(rename = "type")]
    pub condition_type: String,
    pub status: String,
    //pub last_transition_time: Time,
    pub reason: String,
    pub message: String,
}
