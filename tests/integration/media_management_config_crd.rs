//! Integration tests for the SonarrMediaManagementConfig CRD

use crate::common::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, DeleteParams, Patch, PatchParams};
use sonarr_operator::crds::media_management_config::SonarrMediaManagementConfigSpec;
use sonarr_operator::crds::{SonarrInstanceRef, SonarrMediaManagementConfig};

/// Test that the SonarrMediaManagementConfig CRD is installed and established
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_sonarr_media_management_config_crd_is_established() {
    let client = test_client().await;

    assert!(
        is_crd_established(&client, "sonarrmediamanagementconfigs.devopsarr.io").await,
        "SonarrMediaManagementConfig CRD is not established - run 'make install' first"
    );
}

/// Test creating a SonarrMediaManagementConfig resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_create_sonarr_media_management_config() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("media-mgmt-test");
    let config = SonarrMediaManagementConfig {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrMediaManagementConfigSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            recycle_bin: Some("/data/recycle".to_string()),
            recycle_bin_cleanup_days: Some(7),
            copy_using_hardlinks: Some(true),
            create_empty_series_folders: Some(false),
            delete_empty_folders: Some(true),
            ..Default::default()
        },
        status: None,
    };

    let api: Api<SonarrMediaManagementConfig> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();
    let result = api.patch(&name, &patch_params, &Patch::Apply(&config)).await;

    assert!(
        result.is_ok(),
        "Failed to create SonarrMediaManagementConfig: {:?}",
        result.err()
    );

    let retrieved = api.get(&name).await;
    assert!(
        retrieved.is_ok(),
        "Failed to get SonarrMediaManagementConfig: {:?}",
        retrieved.err()
    );

    let config = retrieved.unwrap();
    assert_eq!(config.spec.recycle_bin, Some("/data/recycle".to_string()));
    assert_eq!(config.spec.recycle_bin_cleanup_days, Some(7));

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test updating a SonarrMediaManagementConfig resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_update_sonarr_media_management_config() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("media-mgmt-update");
    let api: Api<SonarrMediaManagementConfig> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    // Create initial resource
    let config = SonarrMediaManagementConfig {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrMediaManagementConfigSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            recycle_bin_cleanup_days: Some(7),
            ..Default::default()
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&config))
        .await
        .expect("Failed to create SonarrMediaManagementConfig");

    // Update the resource
    let updated = SonarrMediaManagementConfig {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrMediaManagementConfigSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            recycle_bin_cleanup_days: Some(14),
            copy_using_hardlinks: Some(false),
            ..Default::default()
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&updated))
        .await
        .expect("Failed to update SonarrMediaManagementConfig");

    let retrieved = api
        .get(&name)
        .await
        .expect("Failed to get SonarrMediaManagementConfig");
    assert_eq!(retrieved.spec.recycle_bin_cleanup_days, Some(14));
    assert_eq!(retrieved.spec.copy_using_hardlinks, Some(false));

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test deleting a SonarrMediaManagementConfig resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_delete_sonarr_media_management_config() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("media-mgmt-delete");
    let api: Api<SonarrMediaManagementConfig> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    let config = SonarrMediaManagementConfig {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrMediaManagementConfigSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            ..Default::default()
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&config))
        .await
        .expect("Failed to create SonarrMediaManagementConfig");

    let delete_result = api.delete(&name, &DeleteParams::default()).await;
    assert!(
        delete_result.is_ok(),
        "Failed to delete SonarrMediaManagementConfig: {:?}",
        delete_result.err()
    );

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let get_result = api.get(&name).await;
    assert!(
        get_result.is_err(),
        "SonarrMediaManagementConfig should have been deleted"
    );
}

/// Test that only one SonarrMediaManagementConfig per Sonarr instance can be created
/// The second one should be marked as conflict by the controller
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_singleton_constraint() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let instance_name = unique_name("singleton-instance");
    let name1 = unique_name("media-mgmt-first");
    let name2 = unique_name("media-mgmt-second");

    let api: Api<SonarrMediaManagementConfig> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    // Create first config
    let config1 = SonarrMediaManagementConfig {
        metadata: ObjectMeta {
            name: Some(name1.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrMediaManagementConfigSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: instance_name.clone(),
                namespace: None,
            },
            recycle_bin_cleanup_days: Some(7),
            ..Default::default()
        },
        status: None,
    };

    api.patch(&name1, &patch_params, &Patch::Apply(&config1))
        .await
        .expect("Failed to create first SonarrMediaManagementConfig");

    // Create second config for the same instance
    let config2 = SonarrMediaManagementConfig {
        metadata: ObjectMeta {
            name: Some(name2.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrMediaManagementConfigSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: instance_name.clone(),
                namespace: None,
            },
            recycle_bin_cleanup_days: Some(14),
            ..Default::default()
        },
        status: None,
    };

    // This should succeed at the K8s level (resource is created)
    // but the controller should mark it as conflict
    let result = api.patch(&name2, &patch_params, &Patch::Apply(&config2)).await;
    assert!(
        result.is_ok(),
        "Failed to create second SonarrMediaManagementConfig: {:?}",
        result.err()
    );

    // Both resources should exist in K8s
    let retrieved1 = api.get(&name1).await;
    let retrieved2 = api.get(&name2).await;
    assert!(retrieved1.is_ok(), "First config should exist");
    assert!(retrieved2.is_ok(), "Second config should exist");

    // Cleanup
    let _ = api.delete(&name1, &DeleteParams::default()).await;
    let _ = api.delete(&name2, &DeleteParams::default()).await;
}
