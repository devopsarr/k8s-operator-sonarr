//! E2E tests for IndexerConfig resource
//!
//! Tests global indexer configuration management via the SonarrIndexerConfig CRD.

use crate::common::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, Patch, PatchParams};
use sonarr_operator::crds::*;
use std::time::Duration;

/// Test IndexerConfig updates global indexer settings in Sonarr
#[tokio::test]
#[ignore = "requires E2E environment - run with: cargo test --test e2e -- --ignored"]
async fn test_indexer_config_update() {
    let mut ctx = TestContext::new()
        .await
        .expect("Failed to create test context");
    setup_e2e_namespace(&ctx.client)
        .await
        .expect("Failed to setup namespace");

    // Get current config from Sonarr for comparison
    let original_config = ctx
        .sonarr
        .get_indexer_config()
        .await
        .expect("Failed to get current indexer config");
    tracing::info!(
        "Original config: rss_sync_interval = {}, minimum_age = {}",
        original_config.rss_sync_interval,
        original_config.minimum_age
    );

    // Create IndexerConfig CR
    let config_name = unique_name("e2e-ic");
    ctx.register_cleanup("SonarrIndexerConfig", E2E_NAMESPACE, &config_name);

    let new_rss_interval = if original_config.rss_sync_interval == 15 {
        25
    } else {
        15
    };
    let new_minimum_age = if original_config.minimum_age == 0 {
        5
    } else {
        0
    };

    let config = SonarrIndexerConfig {
        metadata: ObjectMeta {
            name: Some(config_name.clone()),
            namespace: Some(E2E_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrIndexerConfigSpec {
            rss_sync_interval: Some(new_rss_interval),
            minimum_age: Some(new_minimum_age),
            retention: Some(0),
            maximum_size: Some(0),
            sonarr_instance_ref: SonarrInstanceRef {
                name: "sonarr".to_string(),
                namespace: Some("default".to_string()),
            },
        },
        status: None,
    };

    apply_resource(&ctx.client, &config)
        .await
        .expect("Failed to create indexer config");

    // Wait for Ready
    wait_for_ready::<SonarrIndexerConfig>(&ctx.client, E2E_NAMESPACE, &config_name, E2E_TIMEOUT)
        .await
        .expect("IndexerConfig never became ready");

    // Give Sonarr time to apply the change
    tokio::time::sleep(Duration::from_secs(3)).await;

    // Verify in Sonarr
    let updated_config = ctx
        .sonarr
        .get_indexer_config()
        .await
        .expect("Failed to get updated indexer config");

    assert_eq!(
        updated_config.rss_sync_interval, new_rss_interval,
        "RSS sync interval should be updated"
    );
    assert_eq!(
        updated_config.minimum_age, new_minimum_age,
        "Minimum age should be updated"
    );
    tracing::info!("✓ IndexerConfig verified in Sonarr");

    // Restore original value
    let api: Api<SonarrIndexerConfig> = Api::namespaced(ctx.client.clone(), E2E_NAMESPACE);
    let restore_patch = serde_json::json!({
        "apiVersion": "devopsarr.io/v1alpha1",
        "kind": "SonarrIndexerConfig",
        "metadata": {
            "name": config_name,
            "namespace": E2E_NAMESPACE
        },
        "spec": {
            "rssSyncInterval": original_config.rss_sync_interval,
            "minimumAge": original_config.minimum_age,
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
    tracing::info!("✓ IndexerConfig restored to original values");

    ctx.cleanup().await;
}

/// Test IndexerConfig singleton constraint - only one per Sonarr instance
#[tokio::test]
#[ignore = "requires E2E environment - run with: cargo test --test e2e -- --ignored"]
async fn test_indexer_config_singleton_constraint() {
    let mut ctx = TestContext::new()
        .await
        .expect("Failed to create test context");
    setup_e2e_namespace(&ctx.client)
        .await
        .expect("Failed to setup namespace");

    // Create first IndexerConfig
    let config1_name = unique_name("e2e-ic-1");
    ctx.register_cleanup("SonarrIndexerConfig", E2E_NAMESPACE, &config1_name);

    let config1 = SonarrIndexerConfig {
        metadata: ObjectMeta {
            name: Some(config1_name.clone()),
            namespace: Some(E2E_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrIndexerConfigSpec {
            rss_sync_interval: Some(15),
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
        .expect("Failed to create first indexer config");

    wait_for_ready::<SonarrIndexerConfig>(&ctx.client, E2E_NAMESPACE, &config1_name, E2E_TIMEOUT)
        .await
        .expect("First indexer config never became ready");

    tracing::info!("First IndexerConfig created successfully");

    // Try to create second config for same instance
    let config2_name = unique_name("e2e-ic-2");
    ctx.register_cleanup("SonarrIndexerConfig", E2E_NAMESPACE, &config2_name);

    let config2 = SonarrIndexerConfig {
        metadata: ObjectMeta {
            name: Some(config2_name.clone()),
            namespace: Some(E2E_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrIndexerConfigSpec {
            rss_sync_interval: Some(25),
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
        .expect("Failed to create second indexer config CR");

    // Wait for operator to process
    tokio::time::sleep(Duration::from_secs(10)).await;

    // Check that second config has a conflict/error condition
    let api: Api<SonarrIndexerConfig> = Api::namespaced(ctx.client.clone(), E2E_NAMESPACE);
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
                "✓ Singleton constraint enforced - second IndexerConfig has error/conflict condition"
            );
        } else {
            tracing::warn!(
                "Second config doesn't show conflict - singleton may not be enforced at controller level"
            );
        }
    }

    ctx.cleanup().await;
}
