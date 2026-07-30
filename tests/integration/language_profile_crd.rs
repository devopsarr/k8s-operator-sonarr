//! Integration tests for the SonarrLanguageProfile CRD

use crate::common::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, DeleteParams, Patch, PatchParams};
use sonarr_operator::crds::language_profile::{
    LanguageItem, LanguageType, SonarrLanguageProfileSpec,
};
use sonarr_operator::crds::{SonarrInstanceRef, SonarrLanguageProfile};

/// Test that the SonarrLanguageProfile CRD is installed and established
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_sonarr_language_profile_crd_is_established() {
    let client = test_client().await;

    assert!(
        is_crd_established(&client, "sonarrlanguageprofiles.devopsarr.io").await,
        "SonarrLanguageProfile CRD is not established - run 'make install' first"
    );
}

/// Test creating a SonarrLanguageProfile resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_create_sonarr_language_profile() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("lang-profile-test");
    let profile = SonarrLanguageProfile {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrLanguageProfileSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            name: "Test Language Profile".to_string(),
            upgrade_allowed: true,
            cutoff_language: LanguageType::English,
            languages: vec![
                LanguageItem {
                    language: LanguageType::English,
                    allowed: true,
                },
                LanguageItem {
                    language: LanguageType::French,
                    allowed: true,
                },
            ],
        },
        status: None,
    };

    let api: Api<SonarrLanguageProfile> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();
    let result = api
        .patch(&name, &patch_params, &Patch::Apply(&profile))
        .await;

    assert!(
        result.is_ok(),
        "Failed to create SonarrLanguageProfile: {:?}",
        result.err()
    );

    let retrieved = api.get(&name).await;
    assert!(
        retrieved.is_ok(),
        "Failed to get SonarrLanguageProfile: {:?}",
        retrieved.err()
    );

    let profile = retrieved.unwrap();
    assert_eq!(profile.spec.name, "Test Language Profile");
    assert!(profile.spec.upgrade_allowed);
    assert_eq!(profile.spec.languages.len(), 2);

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test updating a SonarrLanguageProfile resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_update_sonarr_language_profile() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("lang-profile-update");
    let api: Api<SonarrLanguageProfile> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    // Create initial resource
    let profile = SonarrLanguageProfile {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrLanguageProfileSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            name: "Original Profile".to_string(),
            upgrade_allowed: false,
            cutoff_language: LanguageType::English,
            languages: vec![LanguageItem {
                language: LanguageType::English,
                allowed: true,
            }],
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&profile))
        .await
        .expect("Failed to create SonarrLanguageProfile");

    // Update the resource
    let updated = SonarrLanguageProfile {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrLanguageProfileSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            name: "Updated Profile".to_string(),
            upgrade_allowed: true,
            cutoff_language: LanguageType::Japanese,
            languages: vec![
                LanguageItem {
                    language: LanguageType::Japanese,
                    allowed: true,
                },
                LanguageItem {
                    language: LanguageType::English,
                    allowed: true,
                },
            ],
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&updated))
        .await
        .expect("Failed to update SonarrLanguageProfile");

    let retrieved = api
        .get(&name)
        .await
        .expect("Failed to get SonarrLanguageProfile");
    assert_eq!(retrieved.spec.name, "Updated Profile");
    assert!(retrieved.spec.upgrade_allowed);
    assert_eq!(retrieved.spec.languages.len(), 2);

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test deleting a SonarrLanguageProfile resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_delete_sonarr_language_profile() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("lang-profile-delete");
    let api: Api<SonarrLanguageProfile> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    let profile = SonarrLanguageProfile {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrLanguageProfileSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            name: "To Be Deleted".to_string(),
            upgrade_allowed: false,
            cutoff_language: LanguageType::English,
            languages: vec![],
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&profile))
        .await
        .expect("Failed to create SonarrLanguageProfile");

    let delete_result = api.delete(&name, &DeleteParams::default()).await;
    assert!(
        delete_result.is_ok(),
        "Failed to delete SonarrLanguageProfile: {:?}",
        delete_result.err()
    );

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let get_result = api.get(&name).await;
    assert!(
        get_result.is_err(),
        "SonarrLanguageProfile should have been deleted"
    );
}
