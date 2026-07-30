//! SonarrDownloadClientConfig CRD
//!
//! Configures global download client settings for a Sonarr instance.
//! Only one resource per Sonarr instance is allowed.

use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::SonarrInstanceRef;

/// SonarrDownloadClientConfig configures global download client settings for a Sonarr instance.
/// Only one SonarrDownloadClientConfig per Sonarr instance is allowed.
/// Note: This is different from SonarrDownloadClient which configures individual download clients.
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema, Default)]
#[kube(
    group = "devopsarr.io",
    version = "v1alpha1",
    kind = "SonarrDownloadClientConfig",
    plural = "sonarrdownloadclientconfigs",
    shortname = "sdcc",
    namespaced,
    status = "SonarrDownloadClientConfigStatus",
    printcolumn = r#"{"name":"Instance","type":"string","jsonPath":".spec.sonarrInstanceRef.name"}"#,
    printcolumn = r#"{"name":"Completed Handling","type":"boolean","jsonPath":".spec.enableCompletedDownloadHandling"}"#,
    printcolumn = r#"{"name":"Ready","type":"string","jsonPath":".status.conditions[?(@.type==\"Ready\")].status"}"#,
    printcolumn = r#"{"name":"Age","type":"date","jsonPath":".metadata.creationTimestamp"}"#
)]
#[serde(rename_all = "camelCase")]
pub struct SonarrDownloadClientConfigSpec {
    /// Reference to the Sonarr instance
    pub sonarr_instance_ref: SonarrInstanceRef,

    /// Working folders for download client (container path mapping)
    #[serde(default)]
    pub download_client_working_folders: Option<String>,

    /// Enable completed download handling
    #[serde(default)]
    pub enable_completed_download_handling: Option<bool>,

    /// Automatically redownload failed releases
    #[serde(default)]
    pub auto_redownload_failed: Option<bool>,

    /// Automatically redownload failed releases from interactive search
    #[serde(default)]
    pub auto_redownload_failed_from_interactive_search: Option<bool>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SonarrDownloadClientConfigStatus {
    /// Current conditions
    #[serde(default)]
    pub conditions: Vec<Condition>,

    /// Observed generation
    #[serde(default)]
    pub observed_generation: i64,
}
