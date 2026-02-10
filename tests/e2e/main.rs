//! End-to-End tests for the Sonarr Kubernetes Operator
//!
//! These tests verify the full reconciliation loop by:
//! 1. Running the operator against a real Kubernetes cluster
//! 2. Creating CRs and verifying resources are created in Sonarr
//! 3. Testing cross-resource dependencies
//! 4. Verifying cleanup on deletion
//!
//! Prerequisites:
//! 1. A running Kubernetes cluster (e.g., kind, k3d)
//! 2. CRDs installed: `make install`
//! 3. Sonarr instance deployed: `make e2e-sonarr`
//! 4. Operator running: `make run` (in background)
//!
//! Run with: `cargo test --test e2e -- --ignored --test-threads=1`

mod common;
mod scenarios;
mod sonarr_client;

// Re-export for use in scenario tests
pub use common::*;
pub use sonarr_client::SonarrTestClient;
