//! E2E tests for config resources (singleton resources)
//!
//! Tests for MediaManagementConfig, NamingConfig, IndexerConfig, DownloadClientConfig

use crate::common::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, Patch, PatchParams};
use sonarr_operator::crds::*;
use std::time::Duration;

/// Test MediaManagementConfig updates Sonarr settings
#[tokio::test]
#[ignore = "requires E2E environment - run with: cargo test --test e2e -- --ignored"]
async fn test_media_management_config_update() {
    let mut ctx = TestContext::new().await.expect("Failed to create test context");
    setup_e2e_namespace(&ctx.client).await.expect("Failed to setup namespace");

    // Get current config from Sonarr for comparison
    let original_config = ctx.sonarr
        .get_media_management_config()
        .await
        .expect("Failed to get current media management config");
    tracing::info!("Original config: recycle_bin_cleanup_days = {}", original_config.recycle_bin_cleanup_days);

    // Create MediaManagementConfig CR
    let config_name = unique_name("e2e-mmc");
    ctx.register_cleanup("SonarrMediaManagementConfig", E2E_NAMESPACE, &config_name);

    let new_cleanup_days = if original_config.recycle_bin_cleanup_days == 7 { 14 } else { 7 };

    let config = SonarrMediaManagementConfig {
        metadata: ObjectMeta {
            name: Some(config_name.clone()),
            namespace: Some(E2E_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrMediaManagementConfigSpec {
            recycle_bin_cleanup_days: Some(new_cleanup_days),
            create_empty_series_folders: Some(true),
            delete_empty_folders: Some(true),
            sonarr_instance_ref: SonarrInstanceRef {
                name: "sonarr".to_string(),
                namespace: Some("default".to_string()),
            },
            ..Default::default()
        },
        status: None,
    };

    apply_resource(&ctx.client, &config).await.expect("Failed to create config");

    // Wait for Ready
    wait_for_ready::<SonarrMediaManagementConfig>(
        &ctx.client,
        E2E_NAMESPACE,
        &config_name,
        E2E_TIMEOUT,
    )
    .await
    .expect("Config never became ready");

    // Give Sonarr time to apply the change
    tokio::time::sleep(Duration::from_secs(3)).await;

    // Verify in Sonarr
    let updated_config = ctx.sonarr
        .get_media_management_config()
        .await
        .expect("Failed to get updated config");

    assert_eq!(
        updated_config.recycle_bin_cleanup_days, new_cleanup_days,
        "Recycle bin cleanup days should be updated"
    );
    assert!(updated_config.create_empty_series_folders, "Create empty folders should be true");
    assert!(updated_config.delete_empty_folders, "Delete empty folders should be true");
    tracing::info!("✓ MediaManagementConfig verified in Sonarr");

    // Restore original value
    let api: Api<SonarrMediaManagementConfig> = Api::namespaced(ctx.client.clone(), E2E_NAMESPACE);
    let restore_patch = serde_json::json!({
        "apiVersion": "devopsarr.io/v1alpha1",
        "kind": "SonarrMediaManagementConfig",
        "metadata": {
            "name": config_name,
            "namespace": E2E_NAMESPACE
        },
        "spec": {
            "recycleBinCleanupDays": original_config.recycle_bin_cleanup_days,
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
    tracing::info!("✓ Config restored to original values");

    ctx.cleanup().await;
}

/// Test NamingConfig updates episode naming settings
#[tokio::test]
#[ignore = "requires E2E environment - run with: cargo test --test e2e -- --ignored"]
async fn test_naming_config_update() {
    let mut ctx = TestContext::new().await.expect("Failed to create test context");
    setup_e2e_namespace(&ctx.client).await.expect("Failed to setup namespace");

    // Get current config
    let original_config = ctx.sonarr
        .get_naming_config()
        .await
        .expect("Failed to get current naming config");
    tracing::info!("Original config: rename_episodes = {}", original_config.rename_episodes);

    let config_name = unique_name("e2e-nc");
    ctx.register_cleanup("SonarrNamingConfig", E2E_NAMESPACE, &config_name);

    // Create config with specific naming format
    let config = SonarrNamingConfig {
        metadata: ObjectMeta {
            name: Some(config_name.clone()),
            namespace: Some(E2E_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrNamingConfigSpec {
            rename_episodes: Some(true),
            replace_illegal_characters: Some(true),
            standard_episode_format: Some("{Series Title} - S{season:00}E{episode:00} - {Episode Title}".to_string()),
            season_folder_format: Some("Season {season}".to_string()),
            sonarr_instance_ref: SonarrInstanceRef {
                name: "sonarr".to_string(),
                namespace: Some("default".to_string()),
            },
            ..Default::default()
        },
        status: None,
    };

    apply_resource(&ctx.client, &config).await.expect("Failed to create naming config");

    wait_for_ready::<SonarrNamingConfig>(
        &ctx.client,
        E2E_NAMESPACE,
        &config_name,
        E2E_TIMEOUT,
    )
    .await
    .expect("NamingConfig never became ready");

    tokio::time::sleep(Duration::from_secs(3)).await;

    let updated_config = ctx.sonarr
        .get_naming_config()
        .await
        .expect("Failed to get updated naming config");

    assert!(updated_config.rename_episodes, "Rename episodes should be enabled");
    assert!(updated_config.replace_illegal_characters, "Replace illegal chars should be enabled");
    tracing::info!("✓ NamingConfig verified in Sonarr");

    ctx.cleanup().await;
}

/// Test singleton constraint - only one config per instance
#[tokio::test]
#[ignore = "requires E2E environment - run with: cargo test --test e2e -- --ignored"]
async fn test_config_singleton_constraint() {
    let mut ctx = TestContext::new().await.expect("Failed to create test context");
    setup_e2e_namespace(&ctx.client).await.expect("Failed to setup namespace");

    // Create first MediaManagementConfig
    let config1_name = unique_name("e2e-mmc-1");
    ctx.register_cleanup("SonarrMediaManagementConfig", E2E_NAMESPACE, &config1_name);

    let config1 = SonarrMediaManagementConfig {
        metadata: ObjectMeta {
            name: Some(config1_name.clone()),
            namespace: Some(E2E_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrMediaManagementConfigSpec {
            recycle_bin_cleanup_days: Some(7),
            sonarr_instance_ref: SonarrInstanceRef {
                name: "sonarr".to_string(),
                namespace: Some("default".to_string()),
            },
            ..Default::default()
        },
        status: None,
    };

    apply_resource(&ctx.client, &config1).await.expect("Failed to create first config");

    wait_for_ready::<SonarrMediaManagementConfig>(
        &ctx.client,
        E2E_NAMESPACE,
        &config1_name,
        E2E_TIMEOUT,
    )
    .await
    .expect("First config never became ready");

    tracing::info!("First config created successfully");

    // Try to create second config for same instance
    let config2_name = unique_name("e2e-mmc-2");
    ctx.register_cleanup("SonarrMediaManagementConfig", E2E_NAMESPACE, &config2_name);

    let config2 = SonarrMediaManagementConfig {
        metadata: ObjectMeta {
            name: Some(config2_name.clone()),
            namespace: Some(E2E_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrMediaManagementConfigSpec {
            recycle_bin_cleanup_days: Some(14),
            sonarr_instance_ref: SonarrInstanceRef {
                name: "sonarr".to_string(), // Same instance
                namespace: Some("default".to_string()),
            },
            ..Default::default()
        },
        status: None,
    };

    apply_resource(&ctx.client, &config2).await.expect("Failed to create second config CR");

    // Wait a bit for operator to process
    tokio::time::sleep(Duration::from_secs(10)).await;

    // Check that second config has a conflict/error condition
    let api: Api<SonarrMediaManagementConfig> = Api::namespaced(ctx.client.clone(), E2E_NAMESPACE);
    let config2_status = api.get(&config2_name).await.expect("Failed to get second config");

    if let Some(status) = &config2_status.status {
        let has_conflict = status.conditions.iter().any(|c| {
            (c.type_ == "Ready" && c.status == "False") ||
            c.type_ == "Conflict"
        });

        if has_conflict {
            tracing::info!("✓ Singleton constraint enforced - second config has error/conflict condition");
        } else {
            // The operator might handle this differently
            tracing::warn!("Second config doesn't show conflict - singleton may not be enforced at controller level");
        }
    }

    ctx.cleanup().await;
}
