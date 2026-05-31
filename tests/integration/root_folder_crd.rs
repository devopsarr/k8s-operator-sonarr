//! Integration tests for the SonarrRootFolder CRD

use crate::common::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, DeleteParams, Patch, PatchParams};
use sonarr_operator::crds::{SonarrInstanceRef, SonarrRootFolder, SonarrRootFolderSpec};

/// Test that the SonarrRootFolder CRD is installed and established
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_sonarr_root_folder_crd_is_established() {
    let client = test_client().await;

    assert!(
        is_crd_established(&client, "sonarrrootfolders.devopsarr.io").await,
        "SonarrRootFolder CRD is not established - run 'make install' first"
    );
}

/// Test creating a SonarrRootFolder resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_create_sonarr_root_folder() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("rootfolder-test");
    let root_folder = SonarrRootFolder {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrRootFolderSpec {
            path: "/tv".to_string(),
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
        },
        status: None,
    };

    // Create the resource
    let api: Api<SonarrRootFolder> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();
    let result = api
        .patch(&name, &patch_params, &Patch::Apply(&root_folder))
        .await;

    assert!(
        result.is_ok(),
        "Failed to create SonarrRootFolder: {:?}",
        result.err()
    );

    // Verify it exists
    let retrieved = api.get(&name).await;
    assert!(
        retrieved.is_ok(),
        "Failed to get SonarrRootFolder: {:?}",
        retrieved.err()
    );

    let rf = retrieved.unwrap();
    assert_eq!(rf.spec.path, "/tv");
    assert_eq!(rf.spec.sonarr_instance_ref.name, "test-sonarr");

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test creating root folders with different paths
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_create_multiple_root_folders() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let api: Api<SonarrRootFolder> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    let paths = ["/tv/shows", "/tv/anime", "/tv/documentaries"];
    let mut names = Vec::new();

    for (i, path) in paths.iter().enumerate() {
        let name = unique_name(&format!("rootfolder-multi-{}", i));
        names.push(name.clone());

        let root_folder = SonarrRootFolder {
            metadata: ObjectMeta {
                name: Some(name.clone()),
                namespace: Some(TEST_NAMESPACE.to_string()),
                labels: Some([("test-group".to_string(), "multi-rf-test".to_string())].into()),
                ..Default::default()
            },
            spec: SonarrRootFolderSpec {
                path: path.to_string(),
                sonarr_instance_ref: SonarrInstanceRef {
                    name: "test-sonarr".to_string(),
                    namespace: None,
                },
            },
            status: None,
        };

        api.patch(&name, &patch_params, &Patch::Apply(&root_folder))
            .await
            .expect("Failed to create root folder");
    }

    // List with label selector
    let lp = kube::api::ListParams::default().labels("test-group=multi-rf-test");
    let list = api.list(&lp).await.expect("Failed to list root folders");

    assert!(
        list.items.len() >= 3,
        "Expected at least 3 root folders, got {}",
        list.items.len()
    );

    // Cleanup
    for name in &names {
        let _ = api.delete(name, &DeleteParams::default()).await;
    }
}

/// Test updating a SonarrRootFolder path
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_update_sonarr_root_folder() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("rootfolder-update");
    let api: Api<SonarrRootFolder> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    // Create initial resource
    let root_folder = SonarrRootFolder {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrRootFolderSpec {
            path: "/old/path".to_string(),
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&root_folder))
        .await
        .expect("Failed to create SonarrRootFolder");

    // Update the resource
    let updated_rf = SonarrRootFolder {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrRootFolderSpec {
            path: "/new/path".to_string(),
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&updated_rf))
        .await
        .expect("Failed to update SonarrRootFolder");

    // Verify the update
    let retrieved = api
        .get(&name)
        .await
        .expect("Failed to get SonarrRootFolder");
    assert_eq!(retrieved.spec.path, "/new/path");

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test deleting a SonarrRootFolder resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_delete_sonarr_root_folder() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("rootfolder-delete");
    let api: Api<SonarrRootFolder> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    // Create resource
    let root_folder = SonarrRootFolder {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrRootFolderSpec {
            path: "/to-be-deleted".to_string(),
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&root_folder))
        .await
        .expect("Failed to create SonarrRootFolder");

    // Delete the resource
    let delete_result = api.delete(&name, &DeleteParams::default()).await;
    assert!(
        delete_result.is_ok(),
        "Failed to delete SonarrRootFolder: {:?}",
        delete_result.err()
    );

    // Verify it's deleted
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let get_result = api.get(&name).await;
    assert!(
        get_result.is_err(),
        "SonarrRootFolder should have been deleted"
    );
}
