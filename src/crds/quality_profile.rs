use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::SonarrInstanceRef;

/// SonarrQualityProfile represents a quality profile in Sonarr
/// Quality profiles define which qualities are acceptable and their priority
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "devopsarr.io",
    version = "v1alpha1",
    kind = "SonarrQualityProfile",
    plural = "sonarrqualityprofiles",
    shortname = "sqp",
    namespaced,
    status = "SonarrQualityProfileStatus",
    printcolumn = r#"{"name":"Name","type":"string","jsonPath":".spec.name"}"#,
    printcolumn = r#"{"name":"Cutoff","type":"integer","jsonPath":".spec.cutoff"}"#,
    printcolumn = r#"{"name":"ID","type":"integer","jsonPath":".status.id"}"#,
    printcolumn = r#"{"name":"Ready","type":"string","jsonPath":".status.conditions[?(@.type==\"Ready\")].status"}"#,
    printcolumn = r#"{"name":"Age","type":"date","jsonPath":".metadata.creationTimestamp"}"#
)]
#[serde(rename_all = "camelCase")]
pub struct SonarrQualityProfileSpec {
    /// Reference to the SonarrInstance
    pub sonarr_instance_ref: SonarrInstanceRef,

    /// Quality profile name
    pub name: String,

    /// Whether upgrades are allowed
    #[serde(default)]
    pub upgrade_allowed: bool,

    /// Quality ID to use as cutoff
    #[serde(default)]
    pub cutoff: i32,

    /// Cutoff format score
    #[serde(default)]
    pub cutoff_format_score: Option<i32>,

    /// Minimum format score
    #[serde(default)]
    pub min_format_score: Option<i32>,

    /// Minimum upgrade format score
    #[serde(default)]
    pub min_upgrade_format_score: Option<i32>,

    /// Ordered list of quality groups
    pub quality_groups: Vec<QualityGroup>,

    /// Format items (custom formats with scores)
    #[serde(default)]
    pub format_items: Vec<FormatItem>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct QualityGroup {
    /// Quality group ID
    #[serde(default)]
    pub id: Option<i32>,

    /// Quality group name
    #[serde(default)]
    pub name: Option<String>,

    /// Ordered list of qualities in this group
    pub qualities: Vec<Quality>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Quality {
    /// Quality ID
    #[serde(default)]
    pub id: Option<i32>,

    /// Quality name
    #[serde(default)]
    pub name: Option<String>,

    /// Source type
    #[serde(default)]
    pub source: Option<String>,

    /// Resolution
    #[serde(default)]
    pub resolution: Option<i32>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FormatItem {
    /// Custom format ID
    #[serde(default)]
    pub format: Option<i32>,

    /// Format name
    #[serde(default)]
    pub name: Option<String>,

    /// Score for this format
    #[serde(default)]
    pub score: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SonarrQualityProfileStatus {
    /// Current conditions
    #[serde(default)]
    pub conditions: Vec<Condition>,

    /// Sonarr Quality Profile ID
    #[serde(default)]
    pub id: Option<i32>,

    /// Observed generation
    #[serde(default)]
    pub observed_generation: i64,
}
