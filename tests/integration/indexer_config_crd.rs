//! Integration tests for the SonarrIndexerConfig CRD

use crate::common::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, DeleteParams, Patch, PatchParams};
use sonarr_operator::crds::indexer_config::SonarrIndexerConfigSpec;
use sonarr_operator::crds::{SonarrIndexerConfig, SonarrInstanceRef};

/// Test that the SonarrIndexerConfig CRD is installed and established
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_sonarr_indexer_config_crd_is_established() {
    let client = test_client().await;

    assert!(
        is_crd_established(&client, "sonarrindexerconfigs.devopsarr.io").await,
        "SonarrIndexerConfig CRD is not established - run 'make install' first"
    );
}

/// Test creating a SonarrIndexerConfig resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_create_sonarr_indexer_config() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("indexer-cfg-test");
    let config = SonarrIndexerConfig {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrIndexerConfigSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            minimum_age: Some(0),
            retention: Some(0),
            maximum_size: Some(0),
            rss_sync_interval: Some(60),
        },
        status: None,
    };

    let api: Api<SonarrIndexerConfig> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();
    let result = api
        .patch(&name, &patch_params, &Patch::Apply(&config))
        .await;

    assert!(
        result.is_ok(),
        "Failed to create SonarrIndexerConfig: {:?}",
        result.err()
    );

    let retrieved = api.get(&name).await;
    assert!(
        retrieved.is_ok(),
        "Failed to get SonarrIndexerConfig: {:?}",
        retrieved.err()
    );

    let config = retrieved.unwrap();
    assert_eq!(config.spec.rss_sync_interval, Some(60));

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test updating a SonarrIndexerConfig resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_update_sonarr_indexer_config() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("indexer-cfg-update");
    let api: Api<SonarrIndexerConfig> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    // Create initial resource
    let config = SonarrIndexerConfig {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrIndexerConfigSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            rss_sync_interval: Some(30),
            ..Default::default()
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&config))
        .await
        .expect("Failed to create SonarrIndexerConfig");

    // Update the resource
    let updated = SonarrIndexerConfig {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrIndexerConfigSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            rss_sync_interval: Some(120),
            maximum_size: Some(500),
            ..Default::default()
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&updated))
        .await
        .expect("Failed to update SonarrIndexerConfig");

    let retrieved = api
        .get(&name)
        .await
        .expect("Failed to get SonarrIndexerConfig");
    assert_eq!(retrieved.spec.rss_sync_interval, Some(120));
    assert_eq!(retrieved.spec.maximum_size, Some(500));

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test deleting a SonarrIndexerConfig resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_delete_sonarr_indexer_config() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("indexer-cfg-delete");
    let api: Api<SonarrIndexerConfig> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    let config = SonarrIndexerConfig {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrIndexerConfigSpec {
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
        .expect("Failed to create SonarrIndexerConfig");

    let delete_result = api.delete(&name, &DeleteParams::default()).await;
    assert!(
        delete_result.is_ok(),
        "Failed to delete SonarrIndexerConfig: {:?}",
        delete_result.err()
    );

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let get_result = api.get(&name).await;
    assert!(
        get_result.is_err(),
        "SonarrIndexerConfig should have been deleted"
    );
}
