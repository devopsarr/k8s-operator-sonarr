//! E2E tests for CustomFormat lifecycle

use crate::common::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, DeleteParams};
use sonarr_operator::crds::{SonarrCustomFormat, SonarrCustomFormatSpec, SonarrInstanceRef};
use std::time::Duration;

/// Test the full lifecycle of a CustomFormat
#[tokio::test]
#[ignore = "requires E2E environment - run with: cargo test --test e2e -- --ignored"]
async fn test_custom_format_full_lifecycle() {
    let mut ctx = TestContext::new().await.expect("Failed to create test context");
    setup_e2e_namespace(&ctx.client).await.expect("Failed to setup namespace");

    let cf_name = unique_name("e2e-cf");
    let format_name = format!("E2E Custom Format {}", cf_name);

    ctx.register_cleanup("SonarrCustomFormat", E2E_NAMESPACE, &cf_name);

    // Create the custom format CR
    tracing::info!("Creating SonarrCustomFormat: {}", cf_name);
    let custom_format = SonarrCustomFormat {
        metadata: ObjectMeta {
            name: Some(cf_name.clone()),
            namespace: Some(E2E_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrCustomFormatSpec {
            name: format_name.clone(),
            include_custom_format_when_renaming: false,
            specifications: vec![], // Basic format without specifications
            sonarr_instance_ref: SonarrInstanceRef {
                name: "sonarr".to_string(),
                namespace: Some("default".to_string()),
            },
        },
        status: None,
    };

    apply_resource(&ctx.client, &custom_format).await.expect("Failed to create custom format CR");

    // Wait for Ready condition
    tracing::info!("Waiting for custom format to be ready...");
    let ready_cf = wait_for_ready::<SonarrCustomFormat>(
        &ctx.client,
        E2E_NAMESPACE,
        &cf_name,
        E2E_TIMEOUT,
    )
    .await
    .expect("Custom format never became ready");

    let cf_id = ready_cf
        .status
        .as_ref()
        .and_then(|s| s.id)
        .expect("Custom format should have an ID");
    tracing::info!("Custom format created with ID: {}", cf_id);

    // Verify in Sonarr API
    tracing::info!("Verifying custom format exists in Sonarr...");
    let sonarr_cf = ctx.sonarr
        .find_custom_format_by_name(&format_name)
        .await
        .expect("Failed to query Sonarr API")
        .expect("Custom format not found in Sonarr");

    assert_eq!(sonarr_cf.id, cf_id, "Custom format ID mismatch");
    assert_eq!(sonarr_cf.name, format_name, "Custom format name mismatch");
    tracing::info!("✓ Custom format verified in Sonarr");

    // Delete and verify cleanup
    let api: Api<SonarrCustomFormat> = Api::namespaced(ctx.client.clone(), E2E_NAMESPACE);
    api.delete(&cf_name, &DeleteParams::default())
        .await
        .expect("Failed to delete custom format CR");

    wait_for_deletion::<SonarrCustomFormat>(&ctx.client, E2E_NAMESPACE, &cf_name, QUICK_TIMEOUT)
        .await
        .expect("Custom format CR was not deleted");

    tokio::time::sleep(Duration::from_secs(3)).await;

    let deleted = ctx.sonarr.find_custom_format_by_name(&format_name).await;
    assert!(
        matches!(deleted, Ok(None)),
        "Custom format should be deleted from Sonarr"
    );
    tracing::info!("✓ Custom format deleted from Sonarr");

    ctx.cleanup().await;
}
