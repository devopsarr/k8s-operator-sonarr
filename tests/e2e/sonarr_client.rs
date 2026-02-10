//! Sonarr API client for E2E test verification
//!
//! This client directly queries the Sonarr API to verify that resources
//! created by the operator actually exist in Sonarr.

use anyhow::{anyhow, Result};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Client for querying Sonarr API directly during E2E tests
pub struct SonarrTestClient {
    client: reqwest::Client,
    base_url: String,
}

impl SonarrTestClient {
    /// Create a new Sonarr test client
    pub fn new(base_url: &str, api_key: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert("X-Api-Key", HeaderValue::from_str(api_key)?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
        })
    }

    /// Check if Sonarr is healthy
    pub async fn health_check(&self) -> Result<bool> {
        let resp = self
            .client
            .get(format!("{}/api/v3/system/status", self.base_url))
            .send()
            .await?;
        Ok(resp.status().is_success())
    }

    /// Wait for Sonarr to be ready
    pub async fn wait_for_ready(&self, timeout: Duration) -> Result<()> {
        let start = std::time::Instant::now();
        while start.elapsed() < timeout {
            if self.health_check().await.unwrap_or(false) {
                return Ok(());
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
        Err(anyhow!("Timeout waiting for Sonarr to be ready"))
    }

    // ========== Tags ==========

    /// Get all tags from Sonarr
    pub async fn get_tags(&self) -> Result<Vec<Tag>> {
        let resp = self
            .client
            .get(format!("{}/api/v3/tag", self.base_url))
            .send()
            .await?
            .error_for_status()?;
        Ok(resp.json().await?)
    }

    /// Find a tag by label
    pub async fn find_tag_by_label(&self, label: &str) -> Result<Option<Tag>> {
        let tags = self.get_tags().await?;
        Ok(tags.into_iter().find(|t| t.label == label))
    }

    /// Delete a tag by ID
    pub async fn delete_tag(&self, id: i32) -> Result<()> {
        self.client
            .delete(format!("{}/api/v3/tag/{}", self.base_url, id))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    // ========== Root Folders ==========

    /// Get all root folders from Sonarr
    pub async fn get_root_folders(&self) -> Result<Vec<RootFolder>> {
        let resp = self
            .client
            .get(format!("{}/api/v3/rootfolder", self.base_url))
            .send()
            .await?
            .error_for_status()?;
        Ok(resp.json().await?)
    }

    /// Find a root folder by path
    pub async fn find_root_folder_by_path(&self, path: &str) -> Result<Option<RootFolder>> {
        let folders = self.get_root_folders().await?;
        Ok(folders.into_iter().find(|f| f.path == path))
    }

    /// Delete a root folder by ID
    pub async fn delete_root_folder(&self, id: i32) -> Result<()> {
        self.client
            .delete(format!("{}/api/v3/rootfolder/{}", self.base_url, id))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    // ========== Quality Profiles ==========

    /// Get all quality profiles from Sonarr
    pub async fn get_quality_profiles(&self) -> Result<Vec<QualityProfile>> {
        let resp = self
            .client
            .get(format!("{}/api/v3/qualityprofile", self.base_url))
            .send()
            .await?
            .error_for_status()?;
        Ok(resp.json().await?)
    }

    /// Find a quality profile by name
    pub async fn find_quality_profile_by_name(&self, name: &str) -> Result<Option<QualityProfile>> {
        let profiles = self.get_quality_profiles().await?;
        Ok(profiles.into_iter().find(|p| p.name == name))
    }

    /// Delete a quality profile by ID
    pub async fn delete_quality_profile(&self, id: i32) -> Result<()> {
        self.client
            .delete(format!("{}/api/v3/qualityprofile/{}", self.base_url, id))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    // ========== Custom Formats ==========

    /// Get all custom formats from Sonarr
    pub async fn get_custom_formats(&self) -> Result<Vec<CustomFormat>> {
        let resp = self
            .client
            .get(format!("{}/api/v3/customformat", self.base_url))
            .send()
            .await?
            .error_for_status()?;
        Ok(resp.json().await?)
    }

    /// Find a custom format by name
    pub async fn find_custom_format_by_name(&self, name: &str) -> Result<Option<CustomFormat>> {
        let formats = self.get_custom_formats().await?;
        Ok(formats.into_iter().find(|f| f.name == name))
    }

    /// Delete a custom format by ID
    pub async fn delete_custom_format(&self, id: i32) -> Result<()> {
        self.client
            .delete(format!("{}/api/v3/customformat/{}", self.base_url, id))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    // ========== Notifications ==========

    /// Get all notifications from Sonarr
    pub async fn get_notifications(&self) -> Result<Vec<Notification>> {
        let resp = self
            .client
            .get(format!("{}/api/v3/notification", self.base_url))
            .send()
            .await?
            .error_for_status()?;
        Ok(resp.json().await?)
    }

    /// Find a notification by name
    pub async fn find_notification_by_name(&self, name: &str) -> Result<Option<Notification>> {
        let notifications = self.get_notifications().await?;
        Ok(notifications.into_iter().find(|n| n.name == name))
    }

    /// Delete a notification by ID
    pub async fn delete_notification(&self, id: i32) -> Result<()> {
        self.client
            .delete(format!("{}/api/v3/notification/{}", self.base_url, id))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    // ========== Download Clients ==========

    /// Get all download clients from Sonarr
    pub async fn get_download_clients(&self) -> Result<Vec<DownloadClient>> {
        let resp = self
            .client
            .get(format!("{}/api/v3/downloadclient", self.base_url))
            .send()
            .await?
            .error_for_status()?;
        Ok(resp.json().await?)
    }

    /// Find a download client by name
    pub async fn find_download_client_by_name(&self, name: &str) -> Result<Option<DownloadClient>> {
        let clients = self.get_download_clients().await?;
        Ok(clients.into_iter().find(|c| c.name == name))
    }

    /// Delete a download client by ID
    pub async fn delete_download_client(&self, id: i32) -> Result<()> {
        self.client
            .delete(format!("{}/api/v3/downloadclient/{}", self.base_url, id))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    // ========== Indexers ==========

    /// Get all indexers from Sonarr
    pub async fn get_indexers(&self) -> Result<Vec<Indexer>> {
        let resp = self
            .client
            .get(format!("{}/api/v3/indexer", self.base_url))
            .send()
            .await?
            .error_for_status()?;
        Ok(resp.json().await?)
    }

    /// Find an indexer by name
    pub async fn find_indexer_by_name(&self, name: &str) -> Result<Option<Indexer>> {
        let indexers = self.get_indexers().await?;
        Ok(indexers.into_iter().find(|i| i.name == name))
    }

    /// Delete an indexer by ID
    pub async fn delete_indexer(&self, id: i32) -> Result<()> {
        self.client
            .delete(format!("{}/api/v3/indexer/{}", self.base_url, id))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    // ========== Auto Tags ==========

    /// Get all auto tags from Sonarr
    pub async fn get_auto_tags(&self) -> Result<Vec<AutoTag>> {
        let resp = self
            .client
            .get(format!("{}/api/v3/autotag", self.base_url))
            .send()
            .await?
            .error_for_status()?;
        Ok(resp.json().await?)
    }

    /// Find an auto tag by name
    pub async fn find_auto_tag_by_name(&self, name: &str) -> Result<Option<AutoTag>> {
        let auto_tags = self.get_auto_tags().await?;
        Ok(auto_tags.into_iter().find(|a| a.name == name))
    }

    /// Delete an auto tag by ID
    pub async fn delete_auto_tag(&self, id: i32) -> Result<()> {
        self.client
            .delete(format!("{}/api/v3/autotag/{}", self.base_url, id))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    // ========== Media Management Config ==========

    /// Get media management config
    pub async fn get_media_management_config(&self) -> Result<MediaManagementConfig> {
        let resp = self
            .client
            .get(format!("{}/api/v3/config/mediamanagement", self.base_url))
            .send()
            .await?
            .error_for_status()?;
        Ok(resp.json().await?)
    }

    // ========== Naming Config ==========

    /// Get naming config
    pub async fn get_naming_config(&self) -> Result<NamingConfig> {
        let resp = self
            .client
            .get(format!("{}/api/v3/config/naming", self.base_url))
            .send()
            .await?
            .error_for_status()?;
        Ok(resp.json().await?)
    }

    // ========== Delay Profiles ==========

    /// Get all delay profiles from Sonarr
    pub async fn get_delay_profiles(&self) -> Result<Vec<DelayProfile>> {
        let resp = self
            .client
            .get(format!("{}/api/v3/delayprofile", self.base_url))
            .send()
            .await?
            .error_for_status()?;
        Ok(resp.json().await?)
    }

    /// Delete a delay profile by ID
    pub async fn delete_delay_profile(&self, id: i32) -> Result<()> {
        self.client
            .delete(format!("{}/api/v3/delayprofile/{}", self.base_url, id))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}

// ========== API Response Types ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: i32,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootFolder {
    pub id: i32,
    pub path: String,
    #[serde(default)]
    pub accessible: bool,
    #[serde(default)]
    pub free_space: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QualityProfile {
    pub id: i32,
    pub name: String,
    #[serde(default)]
    pub upgrade_allowed: bool,
    #[serde(default)]
    pub cutoff: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomFormat {
    pub id: i32,
    pub name: String,
    #[serde(default)]
    pub include_custom_format_when_renaming: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: i32,
    pub name: String,
    pub implementation: String,
    #[serde(default)]
    pub on_grab: bool,
    #[serde(default)]
    pub on_download: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadClient {
    pub id: i32,
    pub name: String,
    pub implementation: String,
    #[serde(default)]
    pub enable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Indexer {
    pub id: i32,
    pub name: String,
    pub implementation: String,
    #[serde(default)]
    pub enable_rss: bool,
    #[serde(default)]
    pub enable_automatic_search: bool,
    #[serde(default)]
    pub enable_interactive_search: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoTag {
    pub id: i32,
    pub name: String,
    #[serde(default)]
    pub remove_tags_automatically: bool,
    #[serde(default)]
    pub tags: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaManagementConfig {
    pub id: i32,
    #[serde(default)]
    pub auto_unmonitor_previously_downloaded_episodes: bool,
    #[serde(default)]
    pub recycle_bin: String,
    #[serde(default)]
    pub recycle_bin_cleanup_days: i32,
    #[serde(default)]
    pub create_empty_series_folders: bool,
    #[serde(default)]
    pub delete_empty_folders: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamingConfig {
    pub id: i32,
    #[serde(default)]
    pub rename_episodes: bool,
    #[serde(default)]
    pub replace_illegal_characters: bool,
    #[serde(default)]
    pub standard_episode_format: String,
    #[serde(default)]
    pub daily_episode_format: String,
    #[serde(default)]
    pub anime_episode_format: String,
    #[serde(default)]
    pub series_folder_format: String,
    #[serde(default)]
    pub season_folder_format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelayProfile {
    pub id: i32,
    #[serde(default)]
    pub enable_usenet: bool,
    #[serde(default)]
    pub enable_torrent: bool,
    #[serde(default)]
    pub preferred_protocol: String,
    #[serde(default)]
    pub usenet_delay: i32,
    #[serde(default)]
    pub torrent_delay: i32,
    #[serde(default)]
    pub order: i32,
    #[serde(default)]
    pub tags: Vec<i32>,
}
