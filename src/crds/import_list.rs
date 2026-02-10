use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::SonarrInstanceRef;

/// SonarrImportList represents an import list configuration in Sonarr
/// Import lists automatically add series from external sources (Trakt, Plex, etc.)
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "devopsarr.io",
    version = "v1alpha1",
    kind = "SonarrImportList",
    plural = "sonarrimportlists",
    shortname = "sil",
    namespaced,
    status = "SonarrImportListStatus",
    printcolumn = r#"{"name":"Name","type":"string","jsonPath":".spec.name"}"#,
    printcolumn = r#"{"name":"Type","type":"string","jsonPath":".spec.listType"}"#,
    printcolumn = r#"{"name":"ID","type":"integer","jsonPath":".status.id"}"#,
    printcolumn = r#"{"name":"Ready","type":"string","jsonPath":".status.conditions[?(@.type==\"Ready\")].status"}"#,
    printcolumn = r#"{"name":"Age","type":"date","jsonPath":".metadata.creationTimestamp"}"#
)]
#[serde(rename_all = "camelCase")]
pub struct SonarrImportListSpec {
    /// Reference to the SonarrInstance
    pub sonarr_instance_ref: SonarrInstanceRef,

    /// Import list name
    pub name: String,

    /// Import list type/implementation
    pub list_type: ImportListType,

    /// Enable automatic add
    #[serde(default = "default_true")]
    pub enable_automatic_add: bool,

    /// Search for missing episodes when adding
    #[serde(default)]
    pub search_for_missing_episodes: bool,

    /// Monitor type for imported series
    #[serde(default)]
    pub should_monitor: MonitorTypes,

    /// Monitor new items
    #[serde(default)]
    pub monitor_new_items: NewItemMonitorTypes,

    /// Root folder path for imported series
    pub root_folder_path: String,

    /// Quality profile ID to use
    pub quality_profile_id: i32,

    /// Series type
    #[serde(default)]
    pub series_type: SeriesTypes,

    /// Use season folders
    #[serde(default = "default_true")]
    pub season_folder: bool,

    /// List order
    #[serde(default)]
    pub list_order: i32,

    /// Tags for imported series
    #[serde(default)]
    pub tags: Vec<i32>,

    /// Import list configuration
    #[serde(default)]
    pub config: ImportListConfig,
}

fn default_true() -> bool {
    true
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum ImportListType {
    /// Sonarr Import List
    #[default]
    SonarrImport,
    /// Trakt List
    TraktListImport,
    /// Trakt User
    TraktUserImport,
    /// Trakt Popular
    TraktPopularImport,
    /// Plex Watchlist
    PlexImport,
    /// IMDb Lists
    ImdbListImport,
    /// Custom List
    CustomImport,
    /// Simkl
    SimklImport,
    /// AniList
    AniListImport,
    /// MyAnimeList
    MyAnimeListImport,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum MonitorTypes {
    #[default]
    All,
    Future,
    Missing,
    Existing,
    FirstSeason,
    LatestSeason,
    Pilot,
    MonitorSpecials,
    UnmonitorSpecials,
    None,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum NewItemMonitorTypes {
    #[default]
    All,
    None,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum SeriesTypes {
    #[default]
    Standard,
    Daily,
    Anime,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ImportListConfig {
    /// Base URL (for Sonarr import)
    #[serde(default)]
    pub base_url: Option<String>,

    /// API key (for Sonarr import)
    #[serde(default)]
    pub api_key: Option<String>,

    /// Access token (for Trakt/Plex)
    #[serde(default)]
    pub access_token: Option<String>,

    /// Username (for various services)
    #[serde(default)]
    pub username: Option<String>,

    /// Auth user (for Trakt)
    #[serde(default)]
    pub auth_user: Option<String>,

    /// List name/ID
    #[serde(default)]
    pub listname: Option<String>,

    /// List ID
    #[serde(default)]
    pub list_id: Option<String>,

    /// Trakt list type
    #[serde(default)]
    pub trakt_list_type: Option<i32>,

    /// Language profile ID (deprecated in v4)
    #[serde(default)]
    pub language_profile_id: Option<i32>,

    /// Profile IDs (for Sonarr import)
    #[serde(default)]
    pub profile_ids: Vec<i32>,

    /// Tag IDs (for Sonarr import)
    #[serde(default)]
    pub tag_ids: Vec<i32>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SonarrImportListStatus {
    /// Current conditions
    #[serde(default)]
    pub conditions: Vec<Condition>,

    /// Sonarr Import List ID
    #[serde(default)]
    pub id: Option<i32>,

    /// Observed generation
    #[serde(default)]
    pub observed_generation: i64,
}
