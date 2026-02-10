//! Integration tests for the SonarrDelayProfile CRD

use crate::common::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, DeleteParams, Patch, PatchParams};
use sonarr_operator::crds::delay_profile::{DownloadProtocol, SonarrDelayProfileSpec};
use sonarr_operator::crds::{SonarrDelayProfile, SonarrInstanceRef};

/// Test that the SonarrDelayProfile CRD is installed and established
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_sonarr_delay_profile_crd_is_established() {
    let client = test_client().await;

    assert!(
        is_crd_established(&client, "sonarrdelayprofiles.devopsarr.io").await,
        "SonarrDelayProfile CRD is not established - run 'make install' first"
    );
}

/// Test creating a SonarrDelayProfile resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_create_sonarr_delay_profile() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("delay-profile-test");
    let profile = SonarrDelayProfile {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrDelayProfileSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            enable_usenet: true,
            enable_torrent: true,
            preferred_protocol: DownloadProtocol::Usenet,
            usenet_delay: 0,
            torrent_delay: 120,
            bypass_if_highest_quality: true,
            bypass_if_above_custom_format_score: false,
            minimum_custom_format_score: 0,
            order: 1,
            tags: vec![],
        },
        status: None,
    };

    let api: Api<SonarrDelayProfile> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();
    let result = api
        .patch(&name, &patch_params, &Patch::Apply(&profile))
        .await;

    assert!(
        result.is_ok(),
        "Failed to create SonarrDelayProfile: {:?}",
        result.err()
    );

    let retrieved = api.get(&name).await;
    assert!(
        retrieved.is_ok(),
        "Failed to get SonarrDelayProfile: {:?}",
        retrieved.err()
    );

    let profile = retrieved.unwrap();
    assert!(profile.spec.enable_usenet);
    assert_eq!(profile.spec.torrent_delay, 120);
    assert!(profile.spec.bypass_if_highest_quality);

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test updating a SonarrDelayProfile resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_update_sonarr_delay_profile() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("delay-profile-update");
    let api: Api<SonarrDelayProfile> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    // Create initial resource
    let profile = SonarrDelayProfile {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrDelayProfileSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            enable_usenet: true,
            enable_torrent: false,
            preferred_protocol: DownloadProtocol::Usenet,
            usenet_delay: 0,
            torrent_delay: 0,
            bypass_if_highest_quality: false,
            bypass_if_above_custom_format_score: false,
            minimum_custom_format_score: 0,
            order: 1,
            tags: vec![],
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&profile))
        .await
        .expect("Failed to create SonarrDelayProfile");

    // Update the resource
    let updated = SonarrDelayProfile {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrDelayProfileSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            enable_usenet: false,
            enable_torrent: true,
            preferred_protocol: DownloadProtocol::Torrent,
            usenet_delay: 60,
            torrent_delay: 30,
            bypass_if_highest_quality: true,
            bypass_if_above_custom_format_score: true,
            minimum_custom_format_score: 100,
            order: 2,
            tags: vec![1, 2, 3],
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&updated))
        .await
        .expect("Failed to update SonarrDelayProfile");

    let retrieved = api
        .get(&name)
        .await
        .expect("Failed to get SonarrDelayProfile");
    assert!(!retrieved.spec.enable_usenet);
    assert!(retrieved.spec.enable_torrent);
    assert_eq!(retrieved.spec.torrent_delay, 30);
    assert_eq!(retrieved.spec.minimum_custom_format_score, 100);

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test deleting a SonarrDelayProfile resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_delete_sonarr_delay_profile() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("delay-profile-delete");
    let api: Api<SonarrDelayProfile> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    let profile = SonarrDelayProfile {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrDelayProfileSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            enable_usenet: true,
            enable_torrent: true,
            preferred_protocol: DownloadProtocol::Usenet,
            usenet_delay: 0,
            torrent_delay: 0,
            bypass_if_highest_quality: false,
            bypass_if_above_custom_format_score: false,
            minimum_custom_format_score: 0,
            order: 1,
            tags: vec![],
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&profile))
        .await
        .expect("Failed to create SonarrDelayProfile");

    let delete_result = api.delete(&name, &DeleteParams::default()).await;
    assert!(
        delete_result.is_ok(),
        "Failed to delete SonarrDelayProfile: {:?}",
        delete_result.err()
    );

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let get_result = api.get(&name).await;
    assert!(
        get_result.is_err(),
        "SonarrDelayProfile should have been deleted"
    );
}
