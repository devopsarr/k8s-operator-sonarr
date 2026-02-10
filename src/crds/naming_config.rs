//! SonarrNamingConfig CRD
//!
//! Configures episode naming settings for a Sonarr instance.
//! Only one resource per Sonarr instance is allowed.

use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::SonarrInstanceRef;

/// SonarrNamingConfig configures episode naming settings for a Sonarr instance.
/// Only one SonarrNamingConfig per Sonarr instance is allowed.
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema, Default)]
#[kube(
    group = "devopsarr.io",
    version = "v1alpha1",
    kind = "SonarrNamingConfig",
    plural = "sonarrnamingconfigs",
    shortname = "snc",
    namespaced,
    status = "SonarrNamingConfigStatus",
    printcolumn = r#"{"name":"Instance","type":"string","jsonPath":".spec.sonarrInstanceRef.name"}"#,
    printcolumn = r#"{"name":"Rename","type":"boolean","jsonPath":".spec.renameEpisodes"}"#,
    printcolumn = r#"{"name":"Ready","type":"string","jsonPath":".status.conditions[?(@.type==\"Ready\")].status"}"#,
    printcolumn = r#"{"name":"Age","type":"date","jsonPath":".metadata.creationTimestamp"}"#
)]
#[serde(rename_all = "camelCase")]
pub struct SonarrNamingConfigSpec {
    /// Reference to the Sonarr instance
    pub sonarr_instance_ref: SonarrInstanceRef,

    /// Enable episode renaming
    #[serde(default)]
    pub rename_episodes: Option<bool>,

    /// Replace illegal characters in filenames
    #[serde(default)]
    pub replace_illegal_characters: Option<bool>,

    /// Colon replacement format (0=Delete, 1=Dash, 2=SpaceDash, 3=SpaceDashSpace, 4=Smart)
    #[serde(default)]
    pub colon_replacement_format: Option<i32>,

    /// Custom colon replacement format string
    #[serde(default)]
    pub custom_colon_replacement_format: Option<String>,

    /// Multi-episode style (0=Extend, 1=Duplicate, 2=Repeat, 3=Scene, 4=Range, 5=PrefixedRange)
    #[serde(default)]
    pub multi_episode_style: Option<i32>,

    /// Standard episode format
    /// Example: "{Series Title} - S{season:00}E{episode:00} - {Episode Title} {Quality Full}"
    #[serde(default)]
    pub standard_episode_format: Option<String>,

    /// Daily episode format
    /// Example: "{Series Title} - {Air-Date} - {Episode Title} {Quality Full}"
    #[serde(default)]
    pub daily_episode_format: Option<String>,

    /// Anime episode format
    /// Example: "{Series Title} - S{season:00}E{episode:00} - {Episode Title} {Quality Full}"
    #[serde(default)]
    pub anime_episode_format: Option<String>,

    /// Series folder format
    /// Example: "{Series Title}"
    #[serde(default)]
    pub series_folder_format: Option<String>,

    /// Season folder format
    /// Example: "Season {season}"
    #[serde(default)]
    pub season_folder_format: Option<String>,

    /// Specials folder format
    /// Example: "Specials"
    #[serde(default)]
    pub specials_folder_format: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SonarrNamingConfigStatus {
    /// Current conditions
    #[serde(default)]
    pub conditions: Vec<Condition>,

    /// Observed generation
    #[serde(default)]
    pub observed_generation: i64,
}
