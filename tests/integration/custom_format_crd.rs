//! Integration tests for the SonarrCustomFormat CRD

use crate::common::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, DeleteParams, Patch, PatchParams};
use sonarr_operator::crds::custom_format::{
    CustomFormatFields, CustomFormatImplementation, CustomFormatSpecification,
    SonarrCustomFormatSpec,
};
use sonarr_operator::crds::{SonarrCustomFormat, SonarrInstanceRef};

/// Test that the SonarrCustomFormat CRD is installed and established
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_sonarr_custom_format_crd_is_established() {
    let client = test_client().await;

    assert!(
        is_crd_established(&client, "sonarrcustomformats.devopsarr.io").await,
        "SonarrCustomFormat CRD is not established - run 'make install' first"
    );
}

/// Test creating a SonarrCustomFormat resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_create_sonarr_custom_format() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("custom-format-test");
    let cf = SonarrCustomFormat {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrCustomFormatSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            name: "4K HDR".to_string(),
            include_custom_format_when_renaming: true,
            specifications: vec![
                CustomFormatSpecification {
                    name: "4K".to_string(),
                    implementation: CustomFormatImplementation::ResolutionSpecification,
                    negate: false,
                    required: true,
                    fields: CustomFormatFields {
                        value: Some("2160".to_string()),
                        min: None,
                        max: None,
                    },
                },
                CustomFormatSpecification {
                    name: "HDR".to_string(),
                    implementation: CustomFormatImplementation::ReleaseTitleSpecification,
                    negate: false,
                    required: true,
                    fields: CustomFormatFields {
                        value: Some("HDR".to_string()),
                        min: None,
                        max: None,
                    },
                },
            ],
        },
        status: None,
    };

    let api: Api<SonarrCustomFormat> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();
    let result = api.patch(&name, &patch_params, &Patch::Apply(&cf)).await;

    assert!(
        result.is_ok(),
        "Failed to create SonarrCustomFormat: {:?}",
        result.err()
    );

    let retrieved = api.get(&name).await;
    assert!(
        retrieved.is_ok(),
        "Failed to get SonarrCustomFormat: {:?}",
        retrieved.err()
    );

    let cf = retrieved.unwrap();
    assert_eq!(cf.spec.name, "4K HDR");
    assert!(cf.spec.include_custom_format_when_renaming);
    assert_eq!(cf.spec.specifications.len(), 2);

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test updating a SonarrCustomFormat resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_update_sonarr_custom_format() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("custom-format-update");
    let api: Api<SonarrCustomFormat> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    // Create initial resource
    let cf = SonarrCustomFormat {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrCustomFormatSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            name: "Original Format".to_string(),
            include_custom_format_when_renaming: false,
            specifications: vec![],
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&cf))
        .await
        .expect("Failed to create SonarrCustomFormat");

    // Update the resource
    let updated = SonarrCustomFormat {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrCustomFormatSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            name: "Updated Format".to_string(),
            include_custom_format_when_renaming: true,
            specifications: vec![CustomFormatSpecification {
                name: "x265".to_string(),
                implementation: CustomFormatImplementation::ReleaseTitleSpecification,
                negate: false,
                required: true,
                fields: CustomFormatFields {
                    value: Some("x265|HEVC".to_string()),
                    min: None,
                    max: None,
                },
            }],
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&updated))
        .await
        .expect("Failed to update SonarrCustomFormat");

    let retrieved = api
        .get(&name)
        .await
        .expect("Failed to get SonarrCustomFormat");
    assert_eq!(retrieved.spec.name, "Updated Format");
    assert!(retrieved.spec.include_custom_format_when_renaming);
    assert_eq!(retrieved.spec.specifications.len(), 1);

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test deleting a SonarrCustomFormat resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_delete_sonarr_custom_format() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("custom-format-delete");
    let api: Api<SonarrCustomFormat> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    let cf = SonarrCustomFormat {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrCustomFormatSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            name: "To Be Deleted".to_string(),
            include_custom_format_when_renaming: false,
            specifications: vec![],
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&cf))
        .await
        .expect("Failed to create SonarrCustomFormat");

    let delete_result = api.delete(&name, &DeleteParams::default()).await;
    assert!(
        delete_result.is_ok(),
        "Failed to delete SonarrCustomFormat: {:?}",
        delete_result.err()
    );

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let get_result = api.get(&name).await;
    assert!(
        get_result.is_err(),
        "SonarrCustomFormat should have been deleted"
    );
}
