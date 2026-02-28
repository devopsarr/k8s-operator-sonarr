//! E2E tests for RootFolder lifecycle

use crate::common::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, DeleteParams};
use sonarr_operator::crds::{SonarrInstanceRef, SonarrRootFolder, SonarrRootFolderSpec};
use std::time::Duration;

/// Test the full lifecycle of a RootFolder
#[tokio::test]
#[ignore = "requires E2E environment - run with: cargo test --test e2e -- --ignored"]
async fn test_root_folder_full_lifecycle() {
    let mut ctx = TestContext::new()
        .await
        .expect("Failed to create test context");
    setup_e2e_namespace(&ctx.client)
        .await
        .expect("Failed to setup namespace");

    let rf_name = unique_name("e2e-rf");
    // Use /tmp which exists in the Sonarr container (Sonarr validates the path exists)
    let rf_path = "/tmp".to_string();

    ctx.register_cleanup("SonarrRootFolder", E2E_NAMESPACE, &rf_name);

    // Create the root folder CR
    tracing::info!(
        "Creating SonarrRootFolder: {} with path: {}",
        rf_name,
        rf_path
    );
    let root_folder = SonarrRootFolder {
        metadata: ObjectMeta {
            name: Some(rf_name.clone()),
            namespace: Some(E2E_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrRootFolderSpec {
            path: rf_path.clone(),
            sonarr_instance_ref: SonarrInstanceRef {
                name: "sonarr".to_string(),
                namespace: Some("default".to_string()),
            },
        },
        status: None,
    };

    apply_resource(&ctx.client, &root_folder)
        .await
        .expect("Failed to create root folder CR");

    // Wait for Ready condition
    tracing::info!("Waiting for root folder to be ready...");
    let ready_rf =
        wait_for_ready::<SonarrRootFolder>(&ctx.client, E2E_NAMESPACE, &rf_name, E2E_TIMEOUT)
            .await
            .expect("Root folder never became ready");

    let rf_id = ready_rf
        .status
        .as_ref()
        .and_then(|s| s.id)
        .expect("Root folder should have an ID");
    tracing::info!("Root folder created with ID: {}", rf_id);

    // Verify in Sonarr API
    tracing::info!("Verifying root folder exists in Sonarr...");
    let sonarr_rf = ctx
        .sonarr
        .find_root_folder_by_path(&rf_path)
        .await
        .expect("Failed to query Sonarr API")
        .expect("Root folder not found in Sonarr");

    assert_eq!(sonarr_rf.id, rf_id, "Root folder ID mismatch");
    assert_eq!(sonarr_rf.path, rf_path, "Root folder path mismatch");
    tracing::info!("✓ Root folder verified in Sonarr");

    // Delete and verify cleanup
    let api: Api<SonarrRootFolder> = Api::namespaced(ctx.client.clone(), E2E_NAMESPACE);
    api.delete(&rf_name, &DeleteParams::default())
        .await
        .expect("Failed to delete root folder CR");

    wait_for_deletion::<SonarrRootFolder>(&ctx.client, E2E_NAMESPACE, &rf_name, QUICK_TIMEOUT)
        .await
        .expect("Root folder CR was not deleted");

    tokio::time::sleep(Duration::from_secs(3)).await;

    let deleted = ctx.sonarr.find_root_folder_by_path(&rf_path).await;
    assert!(
        matches!(deleted, Ok(None)),
        "Root folder should be deleted from Sonarr"
    );
    tracing::info!("✓ Root folder deleted from Sonarr");

    ctx.cleanup().await;
}
