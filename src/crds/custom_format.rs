use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::SonarrInstanceRef;

/// SonarrCustomFormat represents a custom format configuration in Sonarr
/// Custom formats are used to score releases based on various criteria
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "devopsarr.io",
    version = "v1alpha1",
    kind = "SonarrCustomFormat",
    plural = "sonarrcustomformats",
    shortname = "scf",
    namespaced,
    status = "SonarrCustomFormatStatus",
    printcolumn = r#"{"name":"Name","type":"string","jsonPath":".spec.name"}"#,
    printcolumn = r#"{"name":"ID","type":"integer","jsonPath":".status.id"}"#,
    printcolumn = r#"{"name":"Ready","type":"string","jsonPath":".status.conditions[?(@.type==\"Ready\")].status"}"#,
    printcolumn = r#"{"name":"Age","type":"date","jsonPath":".metadata.creationTimestamp"}"#
)]
#[serde(rename_all = "camelCase")]
pub struct SonarrCustomFormatSpec {
    /// Reference to the SonarrInstance
    pub sonarr_instance_ref: SonarrInstanceRef,

    /// Custom format name
    pub name: String,

    /// Include custom format name when renaming files
    #[serde(default)]
    pub include_custom_format_when_renaming: bool,

    /// Specifications (conditions) for this custom format
    #[serde(default)]
    pub specifications: Vec<CustomFormatSpecification>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CustomFormatSpecification {
    /// Specification name
    pub name: String,

    /// Specification type/implementation
    pub implementation: CustomFormatImplementation,

    /// Negate this condition
    #[serde(default)]
    pub negate: bool,

    /// This condition is required
    #[serde(default = "default_true")]
    pub required: bool,

    /// Fields/values for this specification
    #[serde(default)]
    pub fields: CustomFormatFields,
}

fn default_true() -> bool {
    true
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum CustomFormatImplementation {
    /// Release title matches regex
    #[default]
    ReleaseTitleSpecification,
    /// Source matches
    SourceSpecification,
    /// Resolution matches
    ResolutionSpecification,
    /// Quality modifier matches
    QualityModifierSpecification,
    /// Size specification
    SizeSpecification,
    /// Indexer flag
    IndexerFlagSpecification,
    /// Language matches
    LanguageSpecification,
    /// Release group matches
    ReleaseGroupSpecification,
    /// Edition matches
    EditionSpecification,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CustomFormatFields {
    /// Value for the specification (regex pattern, source type, etc.)
    #[serde(default)]
    pub value: Option<String>,

    /// Minimum value (for size specifications)
    #[serde(default)]
    pub min: Option<f64>,

    /// Maximum value (for size specifications)
    #[serde(default)]
    pub max: Option<f64>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SonarrCustomFormatStatus {
    /// Current conditions
    #[serde(default)]
    pub conditions: Vec<Condition>,

    /// Sonarr Custom Format ID
    #[serde(default)]
    pub id: Option<i32>,

    /// Observed generation
    #[serde(default)]
    pub observed_generation: i64,
}
