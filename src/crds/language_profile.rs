use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::SonarrInstanceRef;

/// SonarrLanguageProfile represents a language profile configuration in Sonarr
/// Language profiles define preferred languages for downloading series
/// Note: Deprecated in Sonarr v4, replaced by per-series language selection
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "devopsarr.io",
    version = "v1alpha1",
    kind = "SonarrLanguageProfile",
    plural = "sonarrlanguageprofiles",
    shortname = "slp",
    namespaced,
    status = "SonarrLanguageProfileStatus",
    printcolumn = r#"{"name":"Name","type":"string","jsonPath":".spec.name"}"#,
    printcolumn = r#"{"name":"Cutoff","type":"string","jsonPath":".spec.cutoffLanguage"}"#,
    printcolumn = r#"{"name":"ID","type":"integer","jsonPath":".status.id"}"#,
    printcolumn = r#"{"name":"Ready","type":"string","jsonPath":".status.conditions[?(@.type==\"Ready\")].status"}"#,
    printcolumn = r#"{"name":"Age","type":"date","jsonPath":".metadata.creationTimestamp"}"#
)]
#[serde(rename_all = "camelCase")]
pub struct SonarrLanguageProfileSpec {
    /// Reference to the SonarrInstance
    pub sonarr_instance_ref: SonarrInstanceRef,

    /// Language profile name
    pub name: String,

    /// Allow upgrades to better quality languages
    #[serde(default)]
    pub upgrade_allowed: bool,

    /// Cutoff language - stop upgrading when this language is reached
    pub cutoff_language: LanguageType,

    /// Ordered list of languages (first = highest priority)
    pub languages: Vec<LanguageItem>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct LanguageItem {
    /// Language
    pub language: LanguageType,

    /// Whether this language is allowed
    #[serde(default = "default_true")]
    pub allowed: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "PascalCase")]
pub enum LanguageType {
    Unknown,
    #[default]
    English,
    French,
    Spanish,
    German,
    Italian,
    Danish,
    Dutch,
    Japanese,
    Icelandic,
    Chinese,
    Russian,
    Polish,
    Vietnamese,
    Swedish,
    Norwegian,
    Finnish,
    Turkish,
    Portuguese,
    Flemish,
    Greek,
    Korean,
    Hungarian,
    Hebrew,
    Lithuanian,
    Czech,
    Hindi,
    Romanian,
    Thai,
    Bulgarian,
    #[serde(rename = "PortugueseBrazil")]
    PortugueseBrazil,
    Arabic,
    Ukrainian,
    Persian,
    Bengali,
    Slovak,
    Latvian,
    #[serde(rename = "SpanishLatino")]
    SpanishLatino,
    Catalan,
    Croatian,
    Serbian,
    Bosnian,
    Estonian,
    Tamil,
    Indonesian,
    Telugu,
    Macedonian,
    Slovenian,
    Malay,
    Original,
    Any,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SonarrLanguageProfileStatus {
    /// Current conditions
    #[serde(default)]
    pub conditions: Vec<Condition>,

    /// Sonarr Language Profile ID
    #[serde(default)]
    pub id: Option<i32>,

    /// Observed generation
    #[serde(default)]
    pub observed_generation: i64,
}
