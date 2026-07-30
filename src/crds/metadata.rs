use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::SonarrInstanceRef;

/// SonarrMetadata represents a metadata consumer configuration in Sonarr
/// Metadata consumers write metadata files for media managers (Kodi, Plex, etc.)
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "devopsarr.io",
    version = "v1alpha1",
    kind = "SonarrMetadata",
    plural = "sonarrmetadatas",
    shortname = "smeta",
    namespaced,
    status = "SonarrMetadataStatus",
    printcolumn = r#"{"name":"Name","type":"string","jsonPath":".spec.name"}"#,
    printcolumn = r#"{"name":"Type","type":"string","jsonPath":".spec.metadataType"}"#,
    printcolumn = r#"{"name":"Enabled","type":"boolean","jsonPath":".spec.enable"}"#,
    printcolumn = r#"{"name":"ID","type":"integer","jsonPath":".status.id"}"#,
    printcolumn = r#"{"name":"Ready","type":"string","jsonPath":".status.conditions[?(@.type==\"Ready\")].status"}"#,
    printcolumn = r#"{"name":"Age","type":"date","jsonPath":".metadata.creationTimestamp"}"#
)]
#[serde(rename_all = "camelCase")]
pub struct SonarrMetadataSpec {
    /// Reference to the SonarrInstance
    pub sonarr_instance_ref: SonarrInstanceRef,

    /// Metadata consumer name
    pub name: String,

    /// Metadata type/implementation
    pub metadata_type: MetadataType,

    /// Enable this metadata consumer
    #[serde(default = "default_true")]
    pub enable: bool,

    /// Tags for this metadata consumer
    #[serde(default)]
    pub tags: Vec<i32>,

    /// Metadata-specific configuration
    #[serde(default)]
    pub config: MetadataConfig,
}

fn default_true() -> bool {
    true
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum MetadataType {
    /// Kodi (XBMC) / Emby metadata
    #[default]
    XbmcMetadata,
    /// Roksbox metadata
    RoksboxMetadata,
    /// WDTV metadata
    WdtvMetadata,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MetadataConfig {
    /// Write series metadata (series.nfo)
    #[serde(default = "default_true")]
    pub series_metadata: bool,

    /// Write series metadata URL (deprecated)
    #[serde(default)]
    pub series_metadata_url: bool,

    /// Write episode metadata (episode.nfo)
    #[serde(default = "default_true")]
    pub episode_metadata: bool,

    /// Write series images (poster, banner, fanart)
    #[serde(default = "default_true")]
    pub series_images: bool,

    /// Write season images
    #[serde(default = "default_true")]
    pub season_images: bool,

    /// Write episode images (thumbnails)
    #[serde(default)]
    pub episode_images: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SonarrMetadataStatus {
    /// Current conditions
    #[serde(default)]
    pub conditions: Vec<Condition>,

    /// Sonarr Metadata ID
    #[serde(default)]
    pub id: Option<i32>,

    /// Observed generation
    #[serde(default)]
    pub observed_generation: i64,
}
