//! Integration tests for the Sonarr Kubernetes Operator
//!
//! These tests require a running Kubernetes cluster accessible via the current kubeconfig context.
//! Run with: `cargo test --test integration -- --ignored`
//!
//! Prerequisites:
//! 1. A running Kubernetes cluster (e.g., kind, k3d, minikube)
//! 2. CRDs installed: `make install`
//! 3. Current kubeconfig context set to the test cluster

mod auto_tag_crd;
mod common;
mod custom_format_crd;
mod delay_profile_crd;
mod download_client_config_crd;
mod import_list_crd;
mod indexer_config_crd;
mod language_profile_crd;
mod media_management_config_crd;
mod metadata_crd;
mod naming_config_crd;
mod quality_definition_crd;
mod root_folder_crd;
mod sonarr_crd;
mod tag_crd;
