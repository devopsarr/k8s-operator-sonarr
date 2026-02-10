//! Sonarr API client factory using the sonarr crate
//!
//! This module provides a factory for creating and caching sonarr Configuration
//! objects for communicating with Sonarr instances.

use sonarr::apis::configuration::{ApiKey, Configuration};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Factory for creating Sonarr API configurations
/// Caches configurations by instance name/namespace to reuse connections
pub struct SonarrClientFactory {
    configs: RwLock<HashMap<String, Arc<Configuration>>>,
}

impl SonarrClientFactory {
    pub fn new() -> Self {
        Self {
            configs: RwLock::new(HashMap::new()),
        }
    }

    /// Get or create a Configuration for the given Sonarr instance
    pub async fn get_config(
        &self,
        url: &str,
        api_key: &str,
        instance_key: &str,
    ) -> Arc<Configuration> {
        // Check if we have a cached config
        {
            let configs = self.configs.read().await;
            if let Some(config) = configs.get(instance_key) {
                return config.clone();
            }
        }

        // Create a new configuration
        let mut config = Configuration::new();
        config.base_path = url.trim_end_matches('/').to_string();
        config.api_key = Some(ApiKey {
            prefix: None,
            key: api_key.to_string(),
        });

        let config = Arc::new(config);

        // Cache it
        {
            let mut configs = self.configs.write().await;
            configs.insert(instance_key.to_string(), config.clone());
        }

        config
    }

    /// Remove a configuration from the cache (e.g., when credentials change)
    pub async fn invalidate(&self, instance_key: &str) {
        let mut configs = self.configs.write().await;
        configs.remove(instance_key);
    }
}

impl Default for SonarrClientFactory {
    fn default() -> Self {
        Self::new()
    }
}

// Re-export sonarr types that controllers will need
pub use sonarr::apis;
pub use sonarr::models;
