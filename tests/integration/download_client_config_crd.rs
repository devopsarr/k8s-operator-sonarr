//! Integration tests for the SonarrDownloadClientConfig CRD

use crate::common::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, DeleteParams, Patch, PatchParams};
use sonarr_operator::crds::download_client_config::SonarrDownloadClientConfigSpec;
use sonarr_operator::crds::{SonarrDownloadClientConfig, SonarrInstanceRef};

/// Test that the SonarrDownloadClientConfig CRD is installed and established
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_sonarr_download_client_config_crd_is_established() {
    let client = test_client().await;

    assert!(
        is_crd_established(&client, "sonarrdownloadclientconfigs.devopsarr.io").await,
        "SonarrDownloadClientConfig CRD is not established - run 'make install' first"
    );
}

/// Test creating a SonarrDownloadClientConfig resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_create_sonarr_download_client_config() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("dlclient-cfg-test");
    let config = SonarrDownloadClientConfig {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrDownloadClientConfigSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            enable_completed_download_handling: Some(true),
            auto_redownload_failed: Some(true),
            auto_redownload_failed_from_interactive_search: Some(false),
            download_client_working_folders: None,
        },
        status: None,
    };

    let api: Api<SonarrDownloadClientConfig> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();
    let result = api
        .patch(&name, &patch_params, &Patch::Apply(&config))
        .await;

    assert!(
        result.is_ok(),
        "Failed to create SonarrDownloadClientConfig: {:?}",
        result.err()
    );

    let retrieved = api.get(&name).await;
    assert!(
        retrieved.is_ok(),
        "Failed to get SonarrDownloadClientConfig: {:?}",
        retrieved.err()
    );

    let config = retrieved.unwrap();
    assert_eq!(config.spec.enable_completed_download_handling, Some(true));
    assert_eq!(config.spec.auto_redownload_failed, Some(true));

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test updating a SonarrDownloadClientConfig resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_update_sonarr_download_client_config() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("dlclient-cfg-update");
    let api: Api<SonarrDownloadClientConfig> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    // Create initial resource
    let config = SonarrDownloadClientConfig {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrDownloadClientConfigSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            enable_completed_download_handling: Some(false),
            ..Default::default()
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&config))
        .await
        .expect("Failed to create SonarrDownloadClientConfig");

    // Update the resource
    let updated = SonarrDownloadClientConfig {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrDownloadClientConfigSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            enable_completed_download_handling: Some(true),
            auto_redownload_failed: Some(true),
            ..Default::default()
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&updated))
        .await
        .expect("Failed to update SonarrDownloadClientConfig");

    let retrieved = api
        .get(&name)
        .await
        .expect("Failed to get SonarrDownloadClientConfig");
    assert_eq!(
        retrieved.spec.enable_completed_download_handling,
        Some(true)
    );
    assert_eq!(retrieved.spec.auto_redownload_failed, Some(true));

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test deleting a SonarrDownloadClientConfig resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_delete_sonarr_download_client_config() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("dlclient-cfg-delete");
    let api: Api<SonarrDownloadClientConfig> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    let config = SonarrDownloadClientConfig {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrDownloadClientConfigSpec {
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
        .expect("Failed to create SonarrDownloadClientConfig");

    let delete_result = api.delete(&name, &DeleteParams::default()).await;
    assert!(
        delete_result.is_ok(),
        "Failed to delete SonarrDownloadClientConfig: {:?}",
        delete_result.err()
    );

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let get_result = api.get(&name).await;
    assert!(
        get_result.is_err(),
        "SonarrDownloadClientConfig should have been deleted"
    );
}
