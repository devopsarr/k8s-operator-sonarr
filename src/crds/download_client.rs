use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::SecretKeySelector;
use super::SonarrInstanceRef;

/// SonarrDownloadClient represents a download client configuration in Sonarr
/// Download clients are used to download releases (qBittorrent, Transmission, SABnzbd, etc.)
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "devopsarr.io",
    version = "v1alpha1",
    kind = "SonarrDownloadClient",
    plural = "sonarrdownloadclients",
    shortname = "sdc",
    namespaced,
    status = "SonarrDownloadClientStatus",
    printcolumn = r#"{"name":"Name","type":"string","jsonPath":".spec.name"}"#,
    printcolumn = r#"{"name":"Type","type":"string","jsonPath":".spec.downloadClientType"}"#,
    printcolumn = r#"{"name":"ID","type":"integer","jsonPath":".status.id"}"#,
    printcolumn = r#"{"name":"Ready","type":"string","jsonPath":".status.conditions[?(@.type==\"Ready\")].status"}"#,
    printcolumn = r#"{"name":"Age","type":"date","jsonPath":".metadata.creationTimestamp"}"#
)]
#[serde(rename_all = "camelCase")]
pub struct SonarrDownloadClientSpec {
    /// Reference to the SonarrInstance
    pub sonarr_instance_ref: SonarrInstanceRef,

    /// Download client name
    pub name: String,

    /// Enable this download client
    #[serde(default = "default_true")]
    pub enable: bool,

    /// Download client type
    pub download_client_type: DownloadClientType,

    /// Priority for this download client
    #[serde(default = "default_priority")]
    pub priority: i32,

    /// Remove completed downloads
    #[serde(default = "default_true")]
    pub remove_completed_downloads: bool,

    /// Remove failed downloads
    #[serde(default = "default_true")]
    pub remove_failed_downloads: bool,

    /// Tags for this download client
    #[serde(default)]
    pub tags: Vec<i32>,

    /// Download client configuration
    pub config: DownloadClientConfig,
}

fn default_true() -> bool {
    true
}

fn default_priority() -> i32 {
    1
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "PascalCase")]
pub enum DownloadClientType {
    Aria2,
    Deluge,
    Flood,
    Hadouken,
    #[serde(rename = "Nzbget")]
    NzbGet,
    #[serde(rename = "Nzbvortex")]
    NzbVortex,
    Pneumatic,
    #[serde(rename = "QBittorrent")]
    QBittorrent,
    #[serde(rename = "RTorrent")]
    RTorrent,
    #[serde(rename = "Sabnzbd")]
    SABnzbd,
    TorrentBlackhole,
    TorrentDownloadStation,
    Transmission,
    UsenetBlackhole,
    UsenetDownloadStation,
    #[serde(rename = "UTorrent")]
    UTorrent,
    Vuze,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DownloadClientConfig {
    /// Host address
    #[serde(default)]
    pub host: Option<String>,

    /// Port number
    #[serde(default)]
    pub port: Option<i32>,

    /// Use SSL
    #[serde(default)]
    pub use_ssl: bool,

    /// URL base path
    #[serde(default)]
    pub url_base: Option<String>,

    /// Username
    #[serde(default)]
    pub username: Option<String>,

    /// Password from secret
    #[serde(default)]
    pub password_secret_ref: Option<SecretKeySelector>,

    /// API key from secret (for some clients)
    #[serde(default)]
    pub api_key_secret_ref: Option<SecretKeySelector>,

    /// TV category
    #[serde(default)]
    pub tv_category: Option<String>,

    /// TV directory
    #[serde(default)]
    pub tv_directory: Option<String>,

    /// Recent TV priority (0 = Last, 1 = First)
    #[serde(default)]
    pub recent_tv_priority: Option<i32>,

    /// Older TV priority (0 = Last, 1 = First)
    #[serde(default)]
    pub older_tv_priority: Option<i32>,

    /// Add paused
    #[serde(default)]
    pub add_paused: bool,

    /// Save magnet files (for blackhole)
    #[serde(default)]
    pub save_magnet_files: bool,

    /// Watch folder (for blackhole)
    #[serde(default)]
    pub watch_folder: Option<String>,

    /// Torrent folder (for blackhole)
    #[serde(default)]
    pub torrent_folder: Option<String>,

    /// NZB folder (for blackhole)
    #[serde(default)]
    pub nzb_folder: Option<String>,

    /// Strm folder (for pneumatic)
    #[serde(default)]
    pub strm_folder: Option<String>,

    /// Secret token (for Aria2)
    #[serde(default)]
    pub secret_token_secret_ref: Option<SecretKeySelector>,

    /// RPC path (for Aria2)
    #[serde(default)]
    pub rpc_path: Option<String>,

    /// Initial state (for qBittorrent: 0 = Start, 1 = ForceStart, 2 = Pause)
    #[serde(default)]
    pub initial_state: Option<i32>,

    /// Sequential order (for qBittorrent)
    #[serde(default)]
    pub sequential_order: bool,

    /// First and last (for qBittorrent)
    #[serde(default)]
    pub first_and_last: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SonarrDownloadClientStatus {
    /// Current conditions
    #[serde(default)]
    pub conditions: Vec<Condition>,

    /// Sonarr Download Client ID
    #[serde(default)]
    pub id: Option<i32>,

    /// Observed generation
    #[serde(default)]
    pub observed_generation: i64,
}
