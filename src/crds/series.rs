use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::SonarrInstanceRef;

/// SonarrSeries represents a TV series managed in Sonarr
/// This allows declarative management of series in your library
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "devopsarr.io",
    version = "v1alpha1",
    kind = "SonarrSeries",
    plural = "sonarrseries",
    shortname = "ss",
    namespaced,
    status = "SonarrSeriesStatus",
    printcolumn = r#"{"name":"Title","type":"string","jsonPath":".spec.title"}"#,
    printcolumn = r#"{"name":"TVDB ID","type":"integer","jsonPath":".spec.tvdbId"}"#,
    printcolumn = r#"{"name":"Monitored","type":"boolean","jsonPath":".spec.monitored"}"#,
    printcolumn = r#"{"name":"ID","type":"integer","jsonPath":".status.id"}"#,
    printcolumn = r#"{"name":"Ready","type":"string","jsonPath":".status.conditions[?(@.type==\"Ready\")].status"}"#,
    printcolumn = r#"{"name":"Age","type":"date","jsonPath":".metadata.creationTimestamp"}"#
)]
#[serde(rename_all = "camelCase")]
pub struct SonarrSeriesSpec {
    /// Reference to the SonarrInstance
    pub sonarr_instance_ref: SonarrInstanceRef,

    /// Series title
    pub title: String,

    /// TVDB ID for the series
    pub tvdb_id: i32,

    /// Title slug (kebab-case version of title)
    pub title_slug: String,

    /// Quality profile ID or name reference
    pub quality_profile: QualityProfileRef,

    /// Root folder path for the series
    pub root_folder_path: String,

    /// Whether the series is monitored
    #[serde(default = "default_true")]
    pub monitored: bool,

    /// Use season folders
    #[serde(default = "default_true")]
    pub season_folder: bool,

    /// Use scene numbering
    #[serde(default)]
    pub use_scene_numbering: bool,

    /// Series type
    #[serde(default = "default_series_type")]
    pub series_type: SeriesType,

    /// Tags for this series
    #[serde(default)]
    pub tags: Vec<i32>,

    /// Specific path override (optional)
    #[serde(default)]
    pub path: Option<String>,

    /// Monitor type for adding series
    #[serde(default = "default_monitor_type")]
    pub add_options: AddSeriesOptions,
}

fn default_true() -> bool {
    true
}

fn default_series_type() -> SeriesType {
    SeriesType::Standard
}

fn default_monitor_type() -> AddSeriesOptions {
    AddSeriesOptions {
        monitor: MonitorType::All,
        search_for_missing_episodes: true,
        search_for_cutoff_unmet_episodes: false,
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct QualityProfileRef {
    /// Quality profile ID
    #[serde(default)]
    pub id: Option<i32>,

    /// Quality profile name (will be resolved to ID)
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum SeriesType {
    Standard,
    Daily,
    Anime,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddSeriesOptions {
    /// Monitor type
    #[serde(default = "default_monitor_all")]
    pub monitor: MonitorType,

    /// Search for missing episodes when adding
    #[serde(default = "default_true_fn")]
    pub search_for_missing_episodes: bool,

    /// Search for cutoff unmet episodes
    #[serde(default)]
    pub search_for_cutoff_unmet_episodes: bool,
}

fn default_monitor_all() -> MonitorType {
    MonitorType::All
}

fn default_true_fn() -> bool {
    true
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum MonitorType {
    /// Monitor all episodes
    All,
    /// Monitor future episodes only
    Future,
    /// Monitor missing episodes only
    Missing,
    /// Monitor existing episodes only
    Existing,
    /// Monitor recent episodes only
    Recent,
    /// Monitor pilot episode only
    Pilot,
    /// Monitor first season only
    FirstSeason,
    /// Monitor last season only
    LastSeason,
    /// Monitor no episodes
    None,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SonarrSeriesStatus {
    /// Current conditions
    #[serde(default)]
    pub conditions: Vec<Condition>,

    /// Sonarr Series ID
    #[serde(default)]
    pub id: Option<i32>,

    /// Observed generation
    #[serde(default)]
    pub observed_generation: i64,

    /// Total episode count
    #[serde(default)]
    pub episode_count: Option<i32>,

    /// Episode file count
    #[serde(default)]
    pub episode_file_count: Option<i32>,

    /// Percentage complete
    #[serde(default)]
    pub percent_complete: Option<f64>,

    /// Next airing date
    #[serde(default)]
    pub next_airing: Option<String>,

    /// Previous airing date
    #[serde(default)]
    pub previous_airing: Option<String>,

    /// Network
    #[serde(default)]
    pub network: Option<String>,

    /// Status (continuing, ended, etc.)
    #[serde(default)]
    pub series_status: Option<String>,
}
