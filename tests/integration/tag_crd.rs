//! Integration tests for the SonarrTag CRD

use crate::common::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, DeleteParams, Patch, PatchParams};
use sonarr_operator::crds::{SonarrInstanceRef, SonarrTag, SonarrTagSpec};

/// Test that the SonarrTag CRD is installed and established
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_sonarr_tag_crd_is_established() {
    let client = test_client().await;

    assert!(
        is_crd_established(&client, "sonarrtags.devopsarr.io").await,
        "SonarrTag CRD is not established - run 'make install' first"
    );
}

/// Test creating a SonarrTag resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_create_sonarr_tag() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("tag-test");
    let tag = SonarrTag {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrTagSpec {
            label: "integration-test".to_string(),
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
        },
        status: None,
    };

    // Create the resource
    let api: Api<SonarrTag> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();
    let result = api.patch(&name, &patch_params, &Patch::Apply(&tag)).await;

    assert!(
        result.is_ok(),
        "Failed to create SonarrTag: {:?}",
        result.err()
    );

    // Verify it exists
    let retrieved = api.get(&name).await;
    assert!(
        retrieved.is_ok(),
        "Failed to get SonarrTag: {:?}",
        retrieved.err()
    );

    let tag = retrieved.unwrap();
    assert_eq!(tag.spec.label, "integration-test");
    assert_eq!(tag.spec.sonarr_instance_ref.name, "test-sonarr");

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test updating a SonarrTag resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_update_sonarr_tag() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("tag-update");
    let api: Api<SonarrTag> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    // Create initial resource
    let tag = SonarrTag {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrTagSpec {
            label: "original-label".to_string(),
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&tag))
        .await
        .expect("Failed to create SonarrTag");

    // Update the resource
    let updated_tag = SonarrTag {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrTagSpec {
            label: "updated-label".to_string(),
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&updated_tag))
        .await
        .expect("Failed to update SonarrTag");

    // Verify the update
    let retrieved = api.get(&name).await.expect("Failed to get SonarrTag");
    assert_eq!(retrieved.spec.label, "updated-label");

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test deleting a SonarrTag resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_delete_sonarr_tag() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("tag-delete");
    let api: Api<SonarrTag> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    // Create resource
    let tag = SonarrTag {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrTagSpec {
            label: "to-be-deleted".to_string(),
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&tag))
        .await
        .expect("Failed to create SonarrTag");

    // Delete the resource
    let delete_result = api.delete(&name, &DeleteParams::default()).await;
    assert!(
        delete_result.is_ok(),
        "Failed to delete SonarrTag: {:?}",
        delete_result.err()
    );

    // Verify it's deleted
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let get_result = api.get(&name).await;
    assert!(get_result.is_err(), "SonarrTag should have been deleted");
}

/// Test creating multiple tags
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_create_multiple_tags() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let api: Api<SonarrTag> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    let tags = vec!["anime", "documentary", "kids"];
    let mut names = Vec::new();

    for tag_label in &tags {
        let name = unique_name(&format!("tag-multi-{}", tag_label));
        names.push(name.clone());

        let tag = SonarrTag {
            metadata: ObjectMeta {
                name: Some(name.clone()),
                namespace: Some(TEST_NAMESPACE.to_string()),
                labels: Some([("test-group".to_string(), "multi-tag-test".to_string())].into()),
                ..Default::default()
            },
            spec: SonarrTagSpec {
                label: tag_label.to_string(),
                sonarr_instance_ref: SonarrInstanceRef {
                    name: "test-sonarr".to_string(),
                    namespace: None,
                },
            },
            status: None,
        };

        api.patch(&name, &patch_params, &Patch::Apply(&tag))
            .await
            .expect(&format!("Failed to create tag {}", tag_label));
    }

    // List with label selector
    let lp = kube::api::ListParams::default().labels("test-group=multi-tag-test");
    let list = api.list(&lp).await.expect("Failed to list tags");

    assert!(
        list.items.len() >= 3,
        "Expected at least 3 tags, got {}",
        list.items.len()
    );

    // Cleanup
    for name in &names {
        let _ = api.delete(name, &DeleteParams::default()).await;
    }
}
