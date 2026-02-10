use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::SonarrInstanceRef;
use super::SecretKeySelector;

/// SonarrNotification represents a notification/connect configuration in Sonarr
/// Notifications are used to alert on events (Discord, Telegram, Webhook, etc.)
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "devopsarr.io",
    version = "v1alpha1",
    kind = "SonarrNotification",
    plural = "sonarrnotifications",
    shortname = "snot",
    namespaced,
    status = "SonarrNotificationStatus",
    printcolumn = r#"{"name":"Name","type":"string","jsonPath":".spec.name"}"#,
    printcolumn = r#"{"name":"Type","type":"string","jsonPath":".spec.notificationType"}"#,
    printcolumn = r#"{"name":"ID","type":"integer","jsonPath":".status.id"}"#,
    printcolumn = r#"{"name":"Ready","type":"string","jsonPath":".status.conditions[?(@.type==\"Ready\")].status"}"#,
    printcolumn = r#"{"name":"Age","type":"date","jsonPath":".metadata.creationTimestamp"}"#
)]
#[serde(rename_all = "camelCase")]
pub struct SonarrNotificationSpec {
    /// Reference to the SonarrInstance
    pub sonarr_instance_ref: SonarrInstanceRef,

    /// Notification name
    pub name: String,

    /// Notification type
    pub notification_type: NotificationType,

    /// Tags for this notification
    #[serde(default)]
    pub tags: Vec<i32>,

    /// Event triggers
    #[serde(default)]
    pub triggers: NotificationTriggers,

    /// Notification configuration
    pub config: NotificationConfig,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "PascalCase")]
pub enum NotificationType {
    Apprise,
    CustomScript,
    Discord,
    Email,
    Emby,
    Gotify,
    Join,
    Kodi,
    Mailgun,
    Ntfy,
    Plex,
    Prowl,
    Pushbullet,
    Pushover,
    SendGrid,
    Signal,
    Simplepush,
    Slack,
    SynologyIndexer,
    Telegram,
    Trakt,
    Twitter,
    Webhook,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotificationTriggers {
    /// On grab (episode is grabbed)
    #[serde(default)]
    pub on_grab: bool,

    /// On download (episode is downloaded)
    #[serde(default)]
    pub on_download: bool,

    /// On upgrade (episode is upgraded)
    #[serde(default)]
    pub on_upgrade: bool,

    /// On rename
    #[serde(default)]
    pub on_rename: bool,

    /// On series add
    #[serde(default)]
    pub on_series_add: bool,

    /// On series delete
    #[serde(default)]
    pub on_series_delete: bool,

    /// On episode file delete
    #[serde(default)]
    pub on_episode_file_delete: bool,

    /// On episode file delete for upgrade
    #[serde(default)]
    pub on_episode_file_delete_for_upgrade: bool,

    /// On health issue
    #[serde(default)]
    pub on_health_issue: bool,

    /// On health restored
    #[serde(default)]
    pub on_health_restored: bool,

    /// On application update
    #[serde(default)]
    pub on_application_update: bool,

    /// On manual interaction required
    #[serde(default)]
    pub on_manual_interaction_required: bool,

    /// On import complete
    #[serde(default)]
    pub on_import_complete: bool,

    /// Include health warnings
    #[serde(default)]
    pub include_health_warnings: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotificationConfig {
    // Webhook configuration
    /// Webhook URL
    #[serde(default)]
    pub url: Option<String>,

    /// HTTP Method (1 = POST, 2 = PUT)
    #[serde(default)]
    pub method: Option<i32>,

    /// Username for basic auth
    #[serde(default)]
    pub username: Option<String>,

    /// Password secret reference
    #[serde(default)]
    pub password_secret_ref: Option<SecretKeySelector>,

    // Discord configuration
    /// Discord webhook URL
    #[serde(default)]
    pub webhook_url: Option<String>,

    /// Discord avatar
    #[serde(default)]
    pub avatar: Option<String>,

    /// Discord username
    #[serde(default)]
    pub discord_username: Option<String>,

    // Telegram configuration
    /// Telegram bot token secret reference
    #[serde(default)]
    pub bot_token_secret_ref: Option<SecretKeySelector>,

    /// Telegram chat ID
    #[serde(default)]
    pub chat_id: Option<String>,

    /// Send silently
    #[serde(default)]
    pub send_silently: bool,

    // Email configuration
    /// SMTP server
    #[serde(default)]
    pub server: Option<String>,

    /// SMTP port
    #[serde(default)]
    pub port: Option<i32>,

    /// Use SSL
    #[serde(default)]
    pub use_ssl: bool,

    /// Require encryption
    #[serde(default)]
    pub require_encryption: bool,

    /// From address
    #[serde(default)]
    pub from: Option<String>,

    /// To addresses
    #[serde(default)]
    pub to: Vec<String>,

    /// CC addresses
    #[serde(default)]
    pub cc: Vec<String>,

    /// BCC addresses
    #[serde(default)]
    pub bcc: Vec<String>,

    // Slack configuration
    /// Slack webhook URL
    #[serde(default)]
    pub slack_webhook_url: Option<String>,

    /// Slack channel
    #[serde(default)]
    pub channel: Option<String>,

    /// Slack icon
    #[serde(default)]
    pub icon: Option<String>,

    // Plex/Emby configuration
    /// Server host
    #[serde(default)]
    pub host: Option<String>,

    /// Auth token secret reference
    #[serde(default)]
    pub auth_token_secret_ref: Option<SecretKeySelector>,

    /// Update library
    #[serde(default)]
    pub update_library: bool,

    /// Notify on specific library sections
    #[serde(default)]
    pub map_to: Option<String>,

    // Gotify configuration
    /// Gotify app token secret reference
    #[serde(default)]
    pub app_token_secret_ref: Option<SecretKeySelector>,

    /// Priority level
    #[serde(default)]
    pub priority: Option<i32>,

    // Pushover configuration
    /// User key secret reference
    #[serde(default)]
    pub user_key_secret_ref: Option<SecretKeySelector>,

    /// API key secret reference
    #[serde(default)]
    pub api_key_secret_ref: Option<SecretKeySelector>,

    /// Device list
    #[serde(default)]
    pub devices: Vec<String>,

    /// Sound
    #[serde(default)]
    pub sound: Option<String>,

    /// Retry interval (seconds)
    #[serde(default)]
    pub retry: Option<i32>,

    /// Expire after (seconds)
    #[serde(default)]
    pub expire: Option<i32>,

    // Custom Script configuration
    /// Path to script
    #[serde(default)]
    pub path: Option<String>,

    /// Script arguments
    #[serde(default)]
    pub arguments: Option<String>,

    // Ntfy configuration
    /// Ntfy server URL
    #[serde(default)]
    pub server_url: Option<String>,

    /// Ntfy topic
    #[serde(default)]
    pub topic: Option<String>,

    /// Click URL
    #[serde(default)]
    pub click_url: Option<String>,

    /// Ntfy tags
    #[serde(default)]
    pub ntfy_tags: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SonarrNotificationStatus {
    /// Current conditions
    #[serde(default)]
    pub conditions: Vec<Condition>,

    /// Sonarr Notification ID
    #[serde(default)]
    pub id: Option<i32>,

    /// Observed generation
    #[serde(default)]
    pub observed_generation: i64,
}
