//! Sonarr Kubernetes Operator Library
//!
//! This library provides the core types and functionality for the Sonarr Kubernetes Operator.
//! It exposes CRDs, controllers, and API clients for managing Sonarr instances via Kubernetes.

pub mod api;
pub mod controllers;
pub mod crds;
pub mod error;

pub use error::{Error, Result};

use crate::api::SonarrClientFactory;
use kube::Client;
use std::sync::Arc;

/// Shared context for all controllers
pub struct Context {
    /// Kubernetes client
    pub client: Client,
    /// Sonarr API client factory
    pub sonarr_client_factory: Arc<SonarrClientFactory>,
}
