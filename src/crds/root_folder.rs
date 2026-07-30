use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::SonarrInstanceRef;

/// SonarrRootFolder represents a root folder in Sonarr
/// Root folders are the base directories where series are stored
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "devopsarr.io",
    version = "v1alpha1",
    kind = "SonarrRootFolder",
    plural = "sonarrrootfolders",
    shortname = "srf",
    namespaced,
    status = "SonarrRootFolderStatus",
    printcolumn = r#"{"name":"Path","type":"string","jsonPath":".spec.path"}"#,
    printcolumn = r#"{"name":"ID","type":"integer","jsonPath":".status.id"}"#,
    printcolumn = r#"{"name":"Ready","type":"string","jsonPath":".status.conditions[?(@.type==\"Ready\")].status"}"#,
    printcolumn = r#"{"name":"Age","type":"date","jsonPath":".metadata.creationTimestamp"}"#
)]
#[serde(rename_all = "camelCase")]
pub struct SonarrRootFolderSpec {
    /// Reference to the SonarrInstance
    pub sonarr_instance_ref: SonarrInstanceRef,

    /// Root folder absolute path
    pub path: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SonarrRootFolderStatus {
    /// Current conditions
    #[serde(default)]
    pub conditions: Vec<Condition>,

    /// Sonarr Root Folder ID
    #[serde(default)]
    pub id: Option<i32>,

    /// Whether the folder is accessible
    #[serde(default)]
    pub accessible: Option<bool>,

    /// Free space in the folder
    #[serde(default)]
    pub free_space: Option<i64>,

    /// Observed generation
    #[serde(default)]
    pub observed_generation: i64,
}
