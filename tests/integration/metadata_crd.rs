//! Integration tests for the SonarrMetadata CRD

use crate::common::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, DeleteParams, Patch, PatchParams};
use sonarr_operator::crds::metadata::{MetadataConfig, MetadataType, SonarrMetadataSpec};
use sonarr_operator::crds::{SonarrInstanceRef, SonarrMetadata};

/// Test that the SonarrMetadata CRD is installed and established
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_sonarr_metadata_crd_is_established() {
    let client = test_client().await;

    assert!(
        is_crd_established(&client, "sonarrmetadatas.devopsarr.io").await,
        "SonarrMetadata CRD is not established - run 'make install' first"
    );
}

/// Test creating a SonarrMetadata resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_create_sonarr_metadata() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("metadata-test");
    let metadata_res = SonarrMetadata {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrMetadataSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            name: "Kodi Metadata".to_string(),
            metadata_type: MetadataType::XbmcMetadata,
            enable: true,
            tags: vec![],
            config: MetadataConfig {
                series_metadata: true,
                series_metadata_url: false,
                episode_metadata: true,
                series_images: true,
                season_images: true,
                episode_images: false,
            },
        },
        status: None,
    };

    let api: Api<SonarrMetadata> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();
    let result = api
        .patch(&name, &patch_params, &Patch::Apply(&metadata_res))
        .await;

    assert!(
        result.is_ok(),
        "Failed to create SonarrMetadata: {:?}",
        result.err()
    );

    let retrieved = api.get(&name).await;
    assert!(
        retrieved.is_ok(),
        "Failed to get SonarrMetadata: {:?}",
        retrieved.err()
    );

    let metadata_res = retrieved.unwrap();
    assert_eq!(metadata_res.spec.name, "Kodi Metadata");
    assert!(metadata_res.spec.enable);

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test updating a SonarrMetadata resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_update_sonarr_metadata() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("metadata-update");
    let api: Api<SonarrMetadata> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    // Create initial resource
    let metadata_res = SonarrMetadata {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrMetadataSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            name: "Original Metadata".to_string(),
            metadata_type: MetadataType::XbmcMetadata,
            enable: true,
            tags: vec![],
            config: MetadataConfig::default(),
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&metadata_res))
        .await
        .expect("Failed to create SonarrMetadata");

    // Update the resource
    let updated = SonarrMetadata {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrMetadataSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            name: "Updated Metadata".to_string(),
            metadata_type: MetadataType::RoksboxMetadata,
            enable: false,
            tags: vec![1],
            config: MetadataConfig {
                series_metadata: false,
                series_metadata_url: true,
                episode_metadata: false,
                series_images: false,
                season_images: false,
                episode_images: true,
            },
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&updated))
        .await
        .expect("Failed to update SonarrMetadata");

    let retrieved = api.get(&name).await.expect("Failed to get SonarrMetadata");
    assert_eq!(retrieved.spec.name, "Updated Metadata");
    assert!(!retrieved.spec.enable);

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test deleting a SonarrMetadata resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_delete_sonarr_metadata() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("metadata-delete");
    let api: Api<SonarrMetadata> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    let metadata_res = SonarrMetadata {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrMetadataSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            name: "To Be Deleted".to_string(),
            metadata_type: MetadataType::WdtvMetadata,
            enable: true,
            tags: vec![],
            config: MetadataConfig::default(),
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&metadata_res))
        .await
        .expect("Failed to create SonarrMetadata");

    let delete_result = api.delete(&name, &DeleteParams::default()).await;
    assert!(
        delete_result.is_ok(),
        "Failed to delete SonarrMetadata: {:?}",
        delete_result.err()
    );

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let get_result = api.get(&name).await;
    assert!(
        get_result.is_err(),
        "SonarrMetadata should have been deleted"
    );
}
