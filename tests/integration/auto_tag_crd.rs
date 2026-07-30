//! Integration tests for the SonarrAutoTag CRD

use crate::common::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, DeleteParams, Patch, PatchParams};
use sonarr_operator::crds::auto_tag::{
    AutoTagFields, AutoTagImplementation, AutoTagSpecification, SonarrAutoTagSpec,
};
use sonarr_operator::crds::{SonarrAutoTag, SonarrInstanceRef};

/// Test that the SonarrAutoTag CRD is installed and established
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_sonarr_auto_tag_crd_is_established() {
    let client = test_client().await;

    assert!(
        is_crd_established(&client, "sonarrautotags.devopsarr.io").await,
        "SonarrAutoTag CRD is not established - run 'make install' first"
    );
}

/// Test creating a SonarrAutoTag resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_create_sonarr_auto_tag() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("auto-tag-test");
    let auto_tag = SonarrAutoTag {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrAutoTagSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            name: "Anime Auto Tag".to_string(),
            remove_tags_automatically: true,
            tags: vec![1, 2],
            specifications: vec![
                AutoTagSpecification {
                    name: "Anime Genre".to_string(),
                    implementation: AutoTagImplementation::GenreSpecification,
                    negate: false,
                    required: true,
                    fields: AutoTagFields {
                        value: Some("Anime".to_string()),
                        min: None,
                        max: None,
                    },
                },
                AutoTagSpecification {
                    name: "Japanese Language".to_string(),
                    implementation: AutoTagImplementation::OriginalLanguageSpecification,
                    negate: false,
                    required: false,
                    fields: AutoTagFields {
                        value: Some("Japanese".to_string()),
                        min: None,
                        max: None,
                    },
                },
            ],
        },
        status: None,
    };

    let api: Api<SonarrAutoTag> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();
    let result = api
        .patch(&name, &patch_params, &Patch::Apply(&auto_tag))
        .await;

    assert!(
        result.is_ok(),
        "Failed to create SonarrAutoTag: {:?}",
        result.err()
    );

    let retrieved = api.get(&name).await;
    assert!(
        retrieved.is_ok(),
        "Failed to get SonarrAutoTag: {:?}",
        retrieved.err()
    );

    let auto_tag = retrieved.unwrap();
    assert_eq!(auto_tag.spec.name, "Anime Auto Tag");
    assert!(auto_tag.spec.remove_tags_automatically);
    assert_eq!(auto_tag.spec.tags.len(), 2);
    assert_eq!(auto_tag.spec.specifications.len(), 2);

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test updating a SonarrAutoTag resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_update_sonarr_auto_tag() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("auto-tag-update");
    let api: Api<SonarrAutoTag> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    // Create initial resource
    let auto_tag = SonarrAutoTag {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrAutoTagSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            name: "Original Auto Tag".to_string(),
            remove_tags_automatically: false,
            tags: vec![1],
            specifications: vec![],
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&auto_tag))
        .await
        .expect("Failed to create SonarrAutoTag");

    // Update the resource
    let updated = SonarrAutoTag {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrAutoTagSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            name: "Updated Auto Tag".to_string(),
            remove_tags_automatically: true,
            tags: vec![1, 2, 3],
            specifications: vec![AutoTagSpecification {
                name: "Root Folder".to_string(),
                implementation: AutoTagImplementation::RootFolderSpecification,
                negate: false,
                required: true,
                fields: AutoTagFields {
                    value: Some("/tv/anime".to_string()),
                    min: None,
                    max: None,
                },
            }],
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&updated))
        .await
        .expect("Failed to update SonarrAutoTag");

    let retrieved = api.get(&name).await.expect("Failed to get SonarrAutoTag");
    assert_eq!(retrieved.spec.name, "Updated Auto Tag");
    assert!(retrieved.spec.remove_tags_automatically);
    assert_eq!(retrieved.spec.tags.len(), 3);
    assert_eq!(retrieved.spec.specifications.len(), 1);

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test deleting a SonarrAutoTag resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_delete_sonarr_auto_tag() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("auto-tag-delete");
    let api: Api<SonarrAutoTag> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    let auto_tag = SonarrAutoTag {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrAutoTagSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            name: "To Be Deleted".to_string(),
            remove_tags_automatically: false,
            tags: vec![],
            specifications: vec![],
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&auto_tag))
        .await
        .expect("Failed to create SonarrAutoTag");

    let delete_result = api.delete(&name, &DeleteParams::default()).await;
    assert!(
        delete_result.is_ok(),
        "Failed to delete SonarrAutoTag: {:?}",
        delete_result.err()
    );

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let get_result = api.get(&name).await;
    assert!(
        get_result.is_err(),
        "SonarrAutoTag should have been deleted"
    );
}

/// Test creating auto tags with different specification types
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_create_auto_tags_with_different_specs() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let api: Api<SonarrAutoTag> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    // Create an auto tag with year specification
    let name = unique_name("auto-tag-year");
    let auto_tag = SonarrAutoTag {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrAutoTagSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            name: "Recent Shows".to_string(),
            remove_tags_automatically: true,
            tags: vec![1],
            specifications: vec![AutoTagSpecification {
                name: "Year Range".to_string(),
                implementation: AutoTagImplementation::YearSpecification,
                negate: false,
                required: true,
                fields: AutoTagFields {
                    value: None,
                    min: Some(2020),
                    max: Some(2030),
                },
            }],
        },
        status: None,
    };

    let result = api
        .patch(&name, &patch_params, &Patch::Apply(&auto_tag))
        .await;
    assert!(
        result.is_ok(),
        "Failed to create auto tag with year spec: {:?}",
        result.err()
    );

    let retrieved = api.get(&name).await.expect("Failed to get auto tag");
    assert_eq!(retrieved.spec.specifications[0].fields.min, Some(2020));
    assert_eq!(retrieved.spec.specifications[0].fields.max, Some(2030));

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}
