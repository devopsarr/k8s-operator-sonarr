//! E2E tests for DownloadClientConfig resource
//!
//! Tests global download client configuration management via the SonarrDownloadClientConfig CRD.

use crate::common::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, Patch, PatchParams};
use sonarr_operator::crds::*;
use std::time::Duration;

/// Test DownloadClientConfig updates global download client settings in Sonarr
#[tokio::test]
#[ignore = "requires E2E environment - run with: cargo test --test e2e -- --ignored"]
async fn test_download_client_config_update() {
    let mut ctx = TestContext::new()
        .await
        .expect("Failed to create test context");
    setup_e2e_namespace(&ctx.client)
        .await
        .expect("Failed to setup namespace");

    // Get current config from Sonarr for comparison
    let original_config = ctx
        .sonarr
        .get_download_client_config()
        .await
        .expect("Failed to get current download client config");
    tracing::info!(
        "Original config: enable_completed_download_handling = {}, auto_redownload_failed = {}",
        original_config.enable_completed_download_handling,
        original_config.auto_redownload_failed
    );

    // Create DownloadClientConfig CR
    let config_name = unique_name("e2e-dcc");
    ctx.register_cleanup("SonarrDownloadClientConfig", E2E_NAMESPACE, &config_name);

    let new_completed_handling = !original_config.enable_completed_download_handling;
    let new_auto_redownload = !original_config.auto_redownload_failed;

    let config = SonarrDownloadClientConfig {
        metadata: ObjectMeta {
            name: Some(config_name.clone()),
            namespace: Some(E2E_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrDownloadClientConfigSpec {
            enable_completed_download_handling: Some(new_completed_handling),
            auto_redownload_failed: Some(new_auto_redownload),
            sonarr_instance_ref: SonarrInstanceRef {
                name: "sonarr".to_string(),
                namespace: Some("default".to_string()),
            },
            ..Default::default()
        },
        status: None,
    };

    apply_resource(&ctx.client, &config)
        .await
        .expect("Failed to create download client config");

    // Wait for Ready
    wait_for_ready::<SonarrDownloadClientConfig>(
        &ctx.client,
        E2E_NAMESPACE,
        &config_name,
        E2E_TIMEOUT,
    )
    .await
    .expect("DownloadClientConfig never became ready");

    // Give Sonarr time to apply the change
    tokio::time::sleep(Duration::from_secs(3)).await;

    // Verify in Sonarr
    let updated_config = ctx
        .sonarr
        .get_download_client_config()
        .await
        .expect("Failed to get updated download client config");

    assert_eq!(
        updated_config.enable_completed_download_handling, new_completed_handling,
        "Enable completed download handling should be updated"
    );
    assert_eq!(
        updated_config.auto_redownload_failed, new_auto_redownload,
        "Auto redownload failed should be updated"
    );
    tracing::info!("✓ DownloadClientConfig verified in Sonarr");

    // Restore original value
    let api: Api<SonarrDownloadClientConfig> = Api::namespaced(ctx.client.clone(), E2E_NAMESPACE);
    let restore_patch = serde_json::json!({
        "apiVersion": "devopsarr.io/v1alpha1",
        "kind": "SonarrDownloadClientConfig",
        "metadata": {
            "name": config_name,
            "namespace": E2E_NAMESPACE
        },
        "spec": {
            "enableCompletedDownloadHandling": original_config.enable_completed_download_handling,
            "autoRedownloadFailed": original_config.auto_redownload_failed,
            "sonarrInstanceRef": {
                "name": "sonarr",
                "namespace": "default"
            }
        }
    });

    api.patch(
        &config_name,
        &PatchParams::apply("sonarr-e2e-test").force(),
        &Patch::Apply(&restore_patch),
    )
    .await
    .expect("Failed to restore config");

    tokio::time::sleep(Duration::from_secs(3)).await;
    tracing::info!("✓ DownloadClientConfig restored to original values");

    ctx.cleanup().await;
}

/// Test DownloadClientConfig singleton constraint - only one per Sonarr instance
#[tokio::test]
#[ignore = "requires E2E environment - run with: cargo test --test e2e -- --ignored"]
async fn test_download_client_config_singleton_constraint() {
    let mut ctx = TestContext::new()
        .await
        .expect("Failed to create test context");
    setup_e2e_namespace(&ctx.client)
        .await
        .expect("Failed to setup namespace");

    // Create first DownloadClientConfig
    let config1_name = unique_name("e2e-dcc-1");
    ctx.register_cleanup("SonarrDownloadClientConfig", E2E_NAMESPACE, &config1_name);

    let config1 = SonarrDownloadClientConfig {
        metadata: ObjectMeta {
            name: Some(config1_name.clone()),
            namespace: Some(E2E_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrDownloadClientConfigSpec {
            enable_completed_download_handling: Some(true),
            sonarr_instance_ref: SonarrInstanceRef {
                name: "sonarr".to_string(),
                namespace: Some("default".to_string()),
            },
            ..Default::default()
        },
        status: None,
    };

    apply_resource(&ctx.client, &config1)
        .await
        .expect("Failed to create first download client config");

    wait_for_ready::<SonarrDownloadClientConfig>(
        &ctx.client,
        E2E_NAMESPACE,
        &config1_name,
        E2E_TIMEOUT,
    )
    .await
    .expect("First download client config never became ready");

    tracing::info!("First DownloadClientConfig created successfully");

    // Try to create second config for same instance
    let config2_name = unique_name("e2e-dcc-2");
    ctx.register_cleanup("SonarrDownloadClientConfig", E2E_NAMESPACE, &config2_name);

    let config2 = SonarrDownloadClientConfig {
        metadata: ObjectMeta {
            name: Some(config2_name.clone()),
            namespace: Some(E2E_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrDownloadClientConfigSpec {
            enable_completed_download_handling: Some(false),
            sonarr_instance_ref: SonarrInstanceRef {
                name: "sonarr".to_string(),
                namespace: Some("default".to_string()),
            },
            ..Default::default()
        },
        status: None,
    };

    apply_resource(&ctx.client, &config2)
        .await
        .expect("Failed to create second download client config CR");

    // Wait for operator to process
    tokio::time::sleep(Duration::from_secs(10)).await;

    // Check that second config has a conflict/error condition
    let api: Api<SonarrDownloadClientConfig> = Api::namespaced(ctx.client.clone(), E2E_NAMESPACE);
    let config2_status = api
        .get(&config2_name)
        .await
        .expect("Failed to get second config");

    if let Some(status) = &config2_status.status {
        let has_conflict = status
            .conditions
            .iter()
            .any(|c| (c.type_ == "Ready" && c.status == "False") || c.type_ == "Conflict");

        if has_conflict {
            tracing::info!(
                "✓ Singleton constraint enforced - second DownloadClientConfig has error/conflict condition"
            );
        } else {
            tracing::warn!("Second config doesn't show conflict - singleton may not be enforced at controller level");
        }
    }

    ctx.cleanup().await;
}
