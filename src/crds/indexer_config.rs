//! SonarrIndexerConfig CRD
//!
//! Configures global indexer settings for a Sonarr instance.
//! Only one resource per Sonarr instance is allowed.

use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::SonarrInstanceRef;

/// SonarrIndexerConfig configures global indexer settings for a Sonarr instance.
/// Only one SonarrIndexerConfig per Sonarr instance is allowed.
/// Note: This is different from SonarrIndexer which configures individual indexers.
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema, Default)]
#[kube(
    group = "devopsarr.io",
    version = "v1alpha1",
    kind = "SonarrIndexerConfig",
    plural = "sonarrindexerconfigs",
    shortname = "sic",
    namespaced,
    status = "SonarrIndexerConfigStatus",
    printcolumn = r#"{"name":"Instance","type":"string","jsonPath":".spec.sonarrInstanceRef.name"}"#,
    printcolumn = r#"{"name":"RSS Interval","type":"integer","jsonPath":".spec.rssSyncInterval"}"#,
    printcolumn = r#"{"name":"Ready","type":"string","jsonPath":".status.conditions[?(@.type==\"Ready\")].status"}"#,
    printcolumn = r#"{"name":"Age","type":"date","jsonPath":".metadata.creationTimestamp"}"#
)]
#[serde(rename_all = "camelCase")]
pub struct SonarrIndexerConfigSpec {
    /// Reference to the Sonarr instance
    pub sonarr_instance_ref: SonarrInstanceRef,

    /// Minimum age in minutes before downloading (usenet)
    #[serde(default)]
    pub minimum_age: Option<i32>,

    /// Retention in days (0 = unlimited)
    #[serde(default)]
    pub retention: Option<i32>,

    /// Maximum release size in MB (0 = unlimited)
    #[serde(default)]
    pub maximum_size: Option<i32>,

    /// RSS sync interval in minutes (0 = disabled, minimum 10)
    #[serde(default)]
    pub rss_sync_interval: Option<i32>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SonarrIndexerConfigStatus {
    /// Current conditions
    #[serde(default)]
    pub conditions: Vec<Condition>,

    /// Observed generation
    #[serde(default)]
    pub observed_generation: i64,
}
