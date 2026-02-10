use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::SonarrInstanceRef;

/// SonarrAutoTag represents an auto-tagging rule configuration in Sonarr
/// Auto-tagging automatically applies tags to series based on conditions
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "devopsarr.io",
    version = "v1alpha1",
    kind = "SonarrAutoTag",
    plural = "sonarrautotags",
    shortname = "sat",
    namespaced,
    status = "SonarrAutoTagStatus",
    printcolumn = r#"{"name":"Name","type":"string","jsonPath":".spec.name"}"#,
    printcolumn = r#"{"name":"RemoveAuto","type":"boolean","jsonPath":".spec.removeTagsAutomatically"}"#,
    printcolumn = r#"{"name":"ID","type":"integer","jsonPath":".status.id"}"#,
    printcolumn = r#"{"name":"Ready","type":"string","jsonPath":".status.conditions[?(@.type==\"Ready\")].status"}"#,
    printcolumn = r#"{"name":"Age","type":"date","jsonPath":".metadata.creationTimestamp"}"#
)]
#[serde(rename_all = "camelCase")]
pub struct SonarrAutoTagSpec {
    /// Reference to the SonarrInstance
    pub sonarr_instance_ref: SonarrInstanceRef,

    /// Auto-tag rule name
    pub name: String,

    /// Remove tags automatically when conditions no longer match
    #[serde(default)]
    pub remove_tags_automatically: bool,

    /// Tags to apply when conditions match
    #[serde(default)]
    pub tags: Vec<i32>,

    /// Specifications (conditions) for this auto-tag rule
    #[serde(default)]
    pub specifications: Vec<AutoTagSpecification>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AutoTagSpecification {
    /// Specification name
    pub name: String,

    /// Specification type/implementation
    pub implementation: AutoTagImplementation,

    /// Negate this condition
    #[serde(default)]
    pub negate: bool,

    /// This condition is required
    #[serde(default = "default_true")]
    pub required: bool,

    /// Fields/values for this specification
    #[serde(default)]
    pub fields: AutoTagFields,
}

fn default_true() -> bool {
    true
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum AutoTagImplementation {
    /// Root folder path matches
    #[default]
    RootFolderSpecification,
    /// Genre matches
    GenreSpecification,
    /// Year matches
    YearSpecification,
    /// Series type matches
    SeriesTypeSpecification,
    /// Quality profile matches
    QualityProfileSpecification,
    /// Network matches
    NetworkSpecification,
    /// Original language matches
    OriginalLanguageSpecification,
    /// Tags match
    TagSpecification,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AutoTagFields {
    /// Value for the specification (path, genre, network, etc.)
    #[serde(default)]
    pub value: Option<String>,

    /// Minimum value (for year specifications)
    #[serde(default)]
    pub min: Option<i32>,

    /// Maximum value (for year specifications)
    #[serde(default)]
    pub max: Option<i32>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SonarrAutoTagStatus {
    /// Current conditions
    #[serde(default)]
    pub conditions: Vec<Condition>,

    /// Sonarr Auto Tag ID
    #[serde(default)]
    pub id: Option<i32>,

    /// Observed generation
    #[serde(default)]
    pub observed_generation: i64,
}
