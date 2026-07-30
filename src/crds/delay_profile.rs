use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::SonarrInstanceRef;

/// SonarrDelayProfile represents a delay profile configuration in Sonarr
/// Delay profiles control how long Sonarr waits before grabbing a release
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "devopsarr.io",
    version = "v1alpha1",
    kind = "SonarrDelayProfile",
    plural = "sonarrdelayprofiles",
    shortname = "sdp",
    namespaced,
    status = "SonarrDelayProfileStatus",
    printcolumn = r#"{"name":"Protocol","type":"string","jsonPath":".spec.preferredProtocol"}"#,
    printcolumn = r#"{"name":"UsenetDelay","type":"integer","jsonPath":".spec.usenetDelay"}"#,
    printcolumn = r#"{"name":"TorrentDelay","type":"integer","jsonPath":".spec.torrentDelay"}"#,
    printcolumn = r#"{"name":"ID","type":"integer","jsonPath":".status.id"}"#,
    printcolumn = r#"{"name":"Ready","type":"string","jsonPath":".status.conditions[?(@.type==\"Ready\")].status"}"#,
    printcolumn = r#"{"name":"Age","type":"date","jsonPath":".metadata.creationTimestamp"}"#
)]
#[serde(rename_all = "camelCase")]
pub struct SonarrDelayProfileSpec {
    /// Reference to the SonarrInstance
    pub sonarr_instance_ref: SonarrInstanceRef,

    /// Enable Usenet downloads
    #[serde(default = "default_true")]
    pub enable_usenet: bool,

    /// Enable Torrent downloads
    #[serde(default = "default_true")]
    pub enable_torrent: bool,

    /// Preferred download protocol
    #[serde(default)]
    pub preferred_protocol: DownloadProtocol,

    /// Delay for Usenet in minutes
    #[serde(default)]
    pub usenet_delay: i32,

    /// Delay for Torrents in minutes
    #[serde(default)]
    pub torrent_delay: i32,

    /// Bypass delay if highest quality
    #[serde(default)]
    pub bypass_if_highest_quality: bool,

    /// Bypass delay if above custom format score
    #[serde(default)]
    pub bypass_if_above_custom_format_score: bool,

    /// Minimum custom format score to bypass delay
    #[serde(default)]
    pub minimum_custom_format_score: i32,

    /// Order of this profile (lower = higher priority)
    #[serde(default)]
    pub order: i32,

    /// Tags to apply this delay profile to
    #[serde(default)]
    pub tags: Vec<i32>,
}

fn default_true() -> bool {
    true
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum DownloadProtocol {
    #[default]
    Usenet,
    Torrent,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SonarrDelayProfileStatus {
    /// Current conditions
    #[serde(default)]
    pub conditions: Vec<Condition>,

    /// Sonarr Delay Profile ID
    #[serde(default)]
    pub id: Option<i32>,

    /// Observed generation
    #[serde(default)]
    pub observed_generation: i64,
}
