use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::SecretKeySelector;
use super::SonarrInstanceRef;

/// SonarrIndexer represents an indexer configuration in Sonarr
/// Indexers are sources for finding releases (Newznab, Torznab, etc.)
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "devopsarr.io",
    version = "v1alpha1",
    kind = "SonarrIndexer",
    plural = "sonarrindexers",
    shortname = "sidx",
    namespaced,
    status = "SonarrIndexerStatus",
    printcolumn = r#"{"name":"Name","type":"string","jsonPath":".spec.name"}"#,
    printcolumn = r#"{"name":"Type","type":"string","jsonPath":".spec.indexerType"}"#,
    printcolumn = r#"{"name":"ID","type":"integer","jsonPath":".status.id"}"#,
    printcolumn = r#"{"name":"Ready","type":"string","jsonPath":".status.conditions[?(@.type==\"Ready\")].status"}"#,
    printcolumn = r#"{"name":"Age","type":"date","jsonPath":".metadata.creationTimestamp"}"#
)]
#[serde(rename_all = "camelCase")]
pub struct SonarrIndexerSpec {
    /// Reference to the SonarrInstance
    pub sonarr_instance_ref: SonarrInstanceRef,

    /// Indexer name
    pub name: String,

    /// Indexer type (Newznab, Torznab, etc.)
    pub indexer_type: IndexerType,

    /// Enable RSS feeds
    #[serde(default = "default_true")]
    pub enable_rss: bool,

    /// Enable automatic search
    #[serde(default = "default_true")]
    pub enable_automatic_search: bool,

    /// Enable interactive search
    #[serde(default = "default_true")]
    pub enable_interactive_search: bool,

    /// Priority for this indexer
    #[serde(default = "default_priority")]
    pub priority: i32,

    /// Download client ID to use
    #[serde(default)]
    pub download_client_id: Option<i32>,

    /// Tags for this indexer
    #[serde(default)]
    pub tags: Vec<i32>,

    /// Indexer-specific configuration
    pub config: IndexerConfig,
}

fn default_true() -> bool {
    true
}

fn default_priority() -> i32 {
    25
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum IndexerType {
    Newznab,
    Torznab,
    Fanzub,
    BroadcastheNet,
    FileList,
    HDBits,
    IPTorrents,
    Nyaa,
    TorrentRss,
    TorrentLeech,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IndexerConfig {
    /// Base URL for the indexer
    #[serde(default)]
    pub base_url: Option<String>,

    /// API path (default: /api)
    #[serde(default)]
    pub api_path: Option<String>,

    /// API key (can reference a secret)
    #[serde(default)]
    pub api_key: Option<String>,

    /// API key from secret reference
    #[serde(default)]
    pub api_key_secret_ref: Option<SecretKeySelector>,

    /// Categories to search
    #[serde(default)]
    pub categories: Vec<i32>,

    /// Anime categories
    #[serde(default)]
    pub anime_categories: Vec<i32>,

    /// Search anime in standard format
    #[serde(default)]
    pub anime_standard_format_search: bool,

    /// Additional parameters
    #[serde(default)]
    pub additional_parameters: Option<String>,

    /// Minimum seeders (for torrent indexers)
    #[serde(default)]
    pub minimum_seeders: Option<i32>,

    /// Seed ratio (for torrent indexers)
    #[serde(default)]
    pub seed_ratio: Option<f64>,

    /// Seed time (for torrent indexers)
    #[serde(default)]
    pub seed_time: Option<i32>,

    /// Cookie (for some indexers)
    #[serde(default)]
    pub cookie: Option<String>,

    /// Username (for some indexers)
    #[serde(default)]
    pub username: Option<String>,

    /// Password secret reference (for some indexers)
    #[serde(default)]
    pub password_secret_ref: Option<SecretKeySelector>,

    /// Passkey (for some indexers)
    #[serde(default)]
    pub passkey: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SonarrIndexerStatus {
    /// Current conditions
    #[serde(default)]
    pub conditions: Vec<Condition>,

    /// Sonarr Indexer ID
    #[serde(default)]
    pub id: Option<i32>,

    /// Observed generation
    #[serde(default)]
    pub observed_generation: i64,
}
