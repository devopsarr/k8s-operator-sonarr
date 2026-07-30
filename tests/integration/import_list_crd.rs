//! Integration tests for the SonarrImportList CRD

use crate::common::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, DeleteParams, Patch, PatchParams};
use sonarr_operator::crds::import_list::{
    ImportListType, MonitorTypes, NewItemMonitorTypes, SeriesTypes, SonarrImportListSpec,
};
use sonarr_operator::crds::{SonarrImportList, SonarrInstanceRef};

/// Test that the SonarrImportList CRD is installed and established
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_sonarr_import_list_crd_is_established() {
    let client = test_client().await;

    assert!(
        is_crd_established(&client, "sonarrimportlists.devopsarr.io").await,
        "SonarrImportList CRD is not established - run 'make install' first"
    );
}

/// Test creating a SonarrImportList resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_create_sonarr_import_list() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("import-list-test");
    let import_list = SonarrImportList {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrImportListSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            name: "Test Import List".to_string(),
            list_type: ImportListType::TraktListImport,
            enable_automatic_add: true,
            search_for_missing_episodes: false,
            should_monitor: MonitorTypes::All,
            monitor_new_items: NewItemMonitorTypes::All,
            root_folder_path: "/tv".to_string(),
            quality_profile_id: 1,
            series_type: SeriesTypes::Standard,
            season_folder: true,
            list_order: 0,
            tags: vec![],
            config: Default::default(),
        },
        status: None,
    };

    let api: Api<SonarrImportList> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();
    let result = api
        .patch(&name, &patch_params, &Patch::Apply(&import_list))
        .await;

    assert!(
        result.is_ok(),
        "Failed to create SonarrImportList: {:?}",
        result.err()
    );

    let retrieved = api.get(&name).await;
    assert!(
        retrieved.is_ok(),
        "Failed to get SonarrImportList: {:?}",
        retrieved.err()
    );

    let import_list = retrieved.unwrap();
    assert_eq!(import_list.spec.name, "Test Import List");
    assert_eq!(import_list.spec.root_folder_path, "/tv");

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test updating a SonarrImportList resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_update_sonarr_import_list() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("import-list-update");
    let api: Api<SonarrImportList> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    // Create initial resource
    let import_list = SonarrImportList {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrImportListSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            name: "Original Import List".to_string(),
            list_type: ImportListType::TraktListImport,
            enable_automatic_add: true,
            search_for_missing_episodes: false,
            should_monitor: MonitorTypes::All,
            monitor_new_items: NewItemMonitorTypes::All,
            root_folder_path: "/tv".to_string(),
            quality_profile_id: 1,
            series_type: SeriesTypes::Standard,
            season_folder: true,
            list_order: 0,
            tags: vec![],
            config: Default::default(),
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&import_list))
        .await
        .expect("Failed to create SonarrImportList");

    // Update the resource
    let updated = SonarrImportList {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrImportListSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            name: "Updated Import List".to_string(),
            list_type: ImportListType::PlexImport,
            enable_automatic_add: false,
            search_for_missing_episodes: true,
            should_monitor: MonitorTypes::Future,
            monitor_new_items: NewItemMonitorTypes::None,
            root_folder_path: "/movies".to_string(),
            quality_profile_id: 2,
            series_type: SeriesTypes::Anime,
            season_folder: false,
            list_order: 1,
            tags: vec![1, 2],
            config: Default::default(),
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&updated))
        .await
        .expect("Failed to update SonarrImportList");

    let retrieved = api
        .get(&name)
        .await
        .expect("Failed to get SonarrImportList");
    assert_eq!(retrieved.spec.name, "Updated Import List");
    assert_eq!(retrieved.spec.root_folder_path, "/movies");

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test deleting a SonarrImportList resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_delete_sonarr_import_list() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("import-list-delete");
    let api: Api<SonarrImportList> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    let import_list = SonarrImportList {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrImportListSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            name: "To Be Deleted".to_string(),
            list_type: ImportListType::SonarrImport,
            enable_automatic_add: true,
            search_for_missing_episodes: false,
            should_monitor: MonitorTypes::All,
            monitor_new_items: NewItemMonitorTypes::All,
            root_folder_path: "/tv".to_string(),
            quality_profile_id: 1,
            series_type: SeriesTypes::Standard,
            season_folder: true,
            list_order: 0,
            tags: vec![],
            config: Default::default(),
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&import_list))
        .await
        .expect("Failed to create SonarrImportList");

    let delete_result = api.delete(&name, &DeleteParams::default()).await;
    assert!(
        delete_result.is_ok(),
        "Failed to delete SonarrImportList: {:?}",
        delete_result.err()
    );

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let get_result = api.get(&name).await;
    assert!(
        get_result.is_err(),
        "SonarrImportList should have been deleted"
    );
}
