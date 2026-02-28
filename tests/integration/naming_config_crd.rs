//! Integration tests for the SonarrNamingConfig CRD

use crate::common::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, DeleteParams, Patch, PatchParams};
use sonarr_operator::crds::naming_config::SonarrNamingConfigSpec;
use sonarr_operator::crds::{SonarrInstanceRef, SonarrNamingConfig};

/// Test that the SonarrNamingConfig CRD is installed and established
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_sonarr_naming_config_crd_is_established() {
    let client = test_client().await;

    assert!(
        is_crd_established(&client, "sonarrnamingconfigs.devopsarr.io").await,
        "SonarrNamingConfig CRD is not established - run 'make install' first"
    );
}

/// Test creating a SonarrNamingConfig resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_create_sonarr_naming_config() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("naming-test");
    let config = SonarrNamingConfig {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrNamingConfigSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            rename_episodes: Some(true),
            replace_illegal_characters: Some(true),
            standard_episode_format: Some(
                "{Series Title} - S{season:00}E{episode:00} - {Episode Title}".to_string(),
            ),
            season_folder_format: Some("Season {season}".to_string()),
            ..Default::default()
        },
        status: None,
    };

    let api: Api<SonarrNamingConfig> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();
    let result = api
        .patch(&name, &patch_params, &Patch::Apply(&config))
        .await;

    assert!(
        result.is_ok(),
        "Failed to create SonarrNamingConfig: {:?}",
        result.err()
    );

    let retrieved = api.get(&name).await;
    assert!(
        retrieved.is_ok(),
        "Failed to get SonarrNamingConfig: {:?}",
        retrieved.err()
    );

    let config = retrieved.unwrap();
    assert_eq!(config.spec.rename_episodes, Some(true));
    assert_eq!(
        config.spec.season_folder_format,
        Some("Season {season}".to_string())
    );

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test updating a SonarrNamingConfig resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_update_sonarr_naming_config() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("naming-update");
    let api: Api<SonarrNamingConfig> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    // Create initial resource
    let config = SonarrNamingConfig {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrNamingConfigSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            rename_episodes: Some(false),
            ..Default::default()
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&config))
        .await
        .expect("Failed to create SonarrNamingConfig");

    // Update the resource
    let updated = SonarrNamingConfig {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrNamingConfigSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            rename_episodes: Some(true),
            standard_episode_format: Some("{Series Title} - {Episode Title}".to_string()),
            ..Default::default()
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&updated))
        .await
        .expect("Failed to update SonarrNamingConfig");

    let retrieved = api
        .get(&name)
        .await
        .expect("Failed to get SonarrNamingConfig");
    assert_eq!(retrieved.spec.rename_episodes, Some(true));
    assert_eq!(
        retrieved.spec.standard_episode_format,
        Some("{Series Title} - {Episode Title}".to_string())
    );

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test deleting a SonarrNamingConfig resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_delete_sonarr_naming_config() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("naming-delete");
    let api: Api<SonarrNamingConfig> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    let config = SonarrNamingConfig {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrNamingConfigSpec {
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
        .expect("Failed to create SonarrNamingConfig");

    let delete_result = api.delete(&name, &DeleteParams::default()).await;
    assert!(
        delete_result.is_ok(),
        "Failed to delete SonarrNamingConfig: {:?}",
        delete_result.err()
    );

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let get_result = api.get(&name).await;
    assert!(
        get_result.is_err(),
        "SonarrNamingConfig should have been deleted"
    );
}
