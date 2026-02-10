use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::SonarrInstanceRef;

/// SonarrQualityDefinition represents a quality definition configuration in Sonarr
/// Quality definitions control the size limits for each quality level
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "devopsarr.io",
    version = "v1alpha1",
    kind = "SonarrQualityDefinition",
    plural = "sonarrqualitydefinitions",
    shortname = "sqd",
    namespaced,
    status = "SonarrQualityDefinitionStatus",
    printcolumn = r#"{"name":"Quality","type":"string","jsonPath":".spec.qualityName"}"#,
    printcolumn = r#"{"name":"Title","type":"string","jsonPath":".spec.title"}"#,
    printcolumn = r#"{"name":"MinSize","type":"number","jsonPath":".spec.minSize"}"#,
    printcolumn = r#"{"name":"MaxSize","type":"number","jsonPath":".spec.maxSize"}"#,
    printcolumn = r#"{"name":"ID","type":"integer","jsonPath":".status.id"}"#,
    printcolumn = r#"{"name":"Ready","type":"string","jsonPath":".status.conditions[?(@.type==\"Ready\")].status"}"#,
    printcolumn = r#"{"name":"Age","type":"date","jsonPath":".metadata.creationTimestamp"}"#
)]
#[serde(rename_all = "camelCase")]
pub struct SonarrQualityDefinitionSpec {
    /// Reference to the SonarrInstance
    pub sonarr_instance_ref: SonarrInstanceRef,

    /// Quality name (must match existing quality in Sonarr)
    pub quality_name: QualityName,

    /// Title/display name for this quality
    #[serde(default)]
    pub title: Option<String>,

    /// Minimum size in MB per minute of runtime
    #[serde(default)]
    pub min_size: Option<f64>,

    /// Maximum size in MB per minute of runtime (None = unlimited)
    #[serde(default)]
    pub max_size: Option<f64>,

    /// Preferred size in MB per minute of runtime
    #[serde(default)]
    pub preferred_size: Option<f64>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QualityName {
    Unknown,
    #[default]
    #[serde(rename = "SDTV")]
    Sdtv,
    #[serde(rename = "DVD")]
    Dvd,
    #[serde(rename = "WEBDL-480p")]
    Webdl480p,
    #[serde(rename = "WEBRip-480p")]
    Webrip480p,
    #[serde(rename = "Bluray-480p")]
    Bluray480p,
    #[serde(rename = "HDTV-720p")]
    Hdtv720p,
    #[serde(rename = "HDTV-1080p")]
    Hdtv1080p,
    #[serde(rename = "Raw-HD")]
    RawHd,
    #[serde(rename = "WEBDL-720p")]
    Webdl720p,
    #[serde(rename = "WEBRip-720p")]
    Webrip720p,
    #[serde(rename = "Bluray-720p")]
    Bluray720p,
    #[serde(rename = "WEBDL-1080p")]
    Webdl1080p,
    #[serde(rename = "WEBRip-1080p")]
    Webrip1080p,
    #[serde(rename = "Bluray-1080p")]
    Bluray1080p,
    #[serde(rename = "Bluray-1080p Remux")]
    Bluray1080pRemux,
    #[serde(rename = "HDTV-2160p")]
    Hdtv2160p,
    #[serde(rename = "WEBDL-2160p")]
    Webdl2160p,
    #[serde(rename = "WEBRip-2160p")]
    Webrip2160p,
    #[serde(rename = "Bluray-2160p")]
    Bluray2160p,
    #[serde(rename = "Bluray-2160p Remux")]
    Bluray2160pRemux,
}

impl QualityName {
    pub fn to_quality_id(&self) -> i32 {
        match self {
            QualityName::Unknown => 0,
            QualityName::Sdtv => 1,
            QualityName::Dvd => 2,
            QualityName::Webdl480p => 8,
            QualityName::Webrip480p => 12,
            QualityName::Bluray480p => 20,
            QualityName::Hdtv720p => 4,
            QualityName::Hdtv1080p => 9,
            QualityName::RawHd => 10,
            QualityName::Webdl720p => 5,
            QualityName::Webrip720p => 14,
            QualityName::Bluray720p => 6,
            QualityName::Webdl1080p => 3,
            QualityName::Webrip1080p => 15,
            QualityName::Bluray1080p => 7,
            QualityName::Bluray1080pRemux => 30,
            QualityName::Hdtv2160p => 16,
            QualityName::Webdl2160p => 18,
            QualityName::Webrip2160p => 17,
            QualityName::Bluray2160p => 19,
            QualityName::Bluray2160pRemux => 31,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SonarrQualityDefinitionStatus {
    /// Current conditions
    #[serde(default)]
    pub conditions: Vec<Condition>,

    /// Sonarr Quality Definition ID
    #[serde(default)]
    pub id: Option<i32>,

    /// Observed generation
    #[serde(default)]
    pub observed_generation: i64,
}
