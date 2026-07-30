//! SonarrMediaManagementConfig CRD
//!
//! Configures media management settings for a Sonarr instance.
//! Only one resource per Sonarr instance is allowed.

use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::SonarrInstanceRef;

/// SonarrMediaManagementConfig configures media management settings for a Sonarr instance.
/// Only one SonarrMediaManagementConfig per Sonarr instance is allowed.
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema, Default)]
#[kube(
    group = "devopsarr.io",
    version = "v1alpha1",
    kind = "SonarrMediaManagementConfig",
    plural = "sonarrmediamanagementconfigs",
    shortname = "smmc",
    namespaced,
    status = "SonarrMediaManagementConfigStatus",
    printcolumn = r#"{"name":"Instance","type":"string","jsonPath":".spec.sonarrInstanceRef.name"}"#,
    printcolumn = r#"{"name":"Ready","type":"string","jsonPath":".status.conditions[?(@.type==\"Ready\")].status"}"#,
    printcolumn = r#"{"name":"Age","type":"date","jsonPath":".metadata.creationTimestamp"}"#
)]
#[serde(rename_all = "camelCase")]
pub struct SonarrMediaManagementConfigSpec {
    /// Reference to the Sonarr instance
    pub sonarr_instance_ref: SonarrInstanceRef,

    /// Auto unmonitor previously downloaded episodes when marked as deleted
    #[serde(default)]
    pub auto_unmonitor_previously_downloaded_episodes: Option<bool>,

    /// Recycle bin path (empty to disable)
    #[serde(default)]
    pub recycle_bin: Option<String>,

    /// Days to keep files in recycle bin before cleaning (0 to disable)
    #[serde(default)]
    pub recycle_bin_cleanup_days: Option<i32>,

    /// Download propers and repacks: DoNotPrefer, PreferAndUpgrade, DoNotUpgrade
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download_propers_and_repacks: Option<ProperDownloadType>,

    /// Create empty series folders during disk scan
    #[serde(default)]
    pub create_empty_series_folders: Option<bool>,

    /// Delete empty series and season folders during disk scan
    #[serde(default)]
    pub delete_empty_folders: Option<bool>,

    /// File date to use: None, LocalAirDate, UtcAirDate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_date: Option<FileDateType>,

    /// Rescan series folder after refresh: Always, AfterManual, Never
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rescan_after_refresh: Option<RescanAfterRefreshType>,

    /// Set permissions on Linux/macOS
    #[serde(default)]
    pub set_permissions_linux: Option<bool>,

    /// chmod folder permissions (e.g., "755")
    #[serde(default)]
    pub chmod_folder: Option<String>,

    /// chown group
    #[serde(default)]
    pub chown_group: Option<String>,

    /// Episode title required: Always, BulkSeasonReleases, Never
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub episode_title_required: Option<EpisodeTitleRequiredType>,

    /// Skip free space check when importing
    #[serde(default)]
    pub skip_free_space_check_when_importing: Option<bool>,

    /// Minimum free space when importing (MB)
    #[serde(default)]
    pub minimum_free_space_when_importing: Option<i32>,

    /// Use hardlinks instead of copy when possible
    #[serde(default)]
    pub copy_using_hardlinks: Option<bool>,

    /// Use script for importing
    #[serde(default)]
    pub use_script_import: Option<bool>,

    /// Script import path
    #[serde(default)]
    pub script_import_path: Option<String>,

    /// Import extra files (subtitles, etc.)
    #[serde(default)]
    pub import_extra_files: Option<bool>,

    /// Extra file extensions to import (e.g., "srt,sub")
    #[serde(default)]
    pub extra_file_extensions: Option<String>,

    /// Enable media info scanning
    #[serde(default)]
    pub enable_media_info: Option<bool>,
}

/// How to handle propers and repacks
#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema, PartialEq)]
pub enum ProperDownloadType {
    #[default]
    DoNotPrefer,
    PreferAndUpgrade,
    DoNotUpgrade,
}

/// File date type to use
#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema, PartialEq)]
pub enum FileDateType {
    #[default]
    None,
    LocalAirDate,
    UtcAirDate,
}

/// When to rescan after refresh
#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema, PartialEq)]
pub enum RescanAfterRefreshType {
    #[default]
    Always,
    AfterManual,
    Never,
}

/// When episode title is required
#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema, PartialEq)]
pub enum EpisodeTitleRequiredType {
    #[default]
    Always,
    BulkSeasonReleases,
    Never,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SonarrMediaManagementConfigStatus {
    /// Current conditions
    #[serde(default)]
    pub conditions: Vec<Condition>,

    /// Observed generation
    #[serde(default)]
    pub observed_generation: i64,
}
