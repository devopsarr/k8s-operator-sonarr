//! Integration tests for the SonarrQualityDefinition CRD

use crate::common::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, DeleteParams, Patch, PatchParams};
use sonarr_operator::crds::quality_definition::{QualityName, SonarrQualityDefinitionSpec};
use sonarr_operator::crds::{SonarrInstanceRef, SonarrQualityDefinition};

/// Test that the SonarrQualityDefinition CRD is installed and established
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_sonarr_quality_definition_crd_is_established() {
    let client = test_client().await;

    assert!(
        is_crd_established(&client, "sonarrqualitydefinitions.devopsarr.io").await,
        "SonarrQualityDefinition CRD is not established - run 'make install' first"
    );
}

/// Test creating a SonarrQualityDefinition resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_create_sonarr_quality_definition() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("quality-def-test");
    let qd = SonarrQualityDefinition {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrQualityDefinitionSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            quality_name: QualityName::Bluray1080p,
            title: Some("Bluray 1080p Custom".to_string()),
            min_size: Some(10.0),
            max_size: Some(100.0),
            preferred_size: Some(50.0),
        },
        status: None,
    };

    let api: Api<SonarrQualityDefinition> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();
    let result = api.patch(&name, &patch_params, &Patch::Apply(&qd)).await;

    assert!(
        result.is_ok(),
        "Failed to create SonarrQualityDefinition: {:?}",
        result.err()
    );

    let retrieved = api.get(&name).await;
    assert!(
        retrieved.is_ok(),
        "Failed to get SonarrQualityDefinition: {:?}",
        retrieved.err()
    );

    let qd = retrieved.unwrap();
    assert_eq!(qd.spec.title, Some("Bluray 1080p Custom".to_string()));
    assert_eq!(qd.spec.min_size, Some(10.0));
    assert_eq!(qd.spec.max_size, Some(100.0));

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test updating a SonarrQualityDefinition resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_update_sonarr_quality_definition() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("quality-def-update");
    let api: Api<SonarrQualityDefinition> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    // Create initial resource
    let qd = SonarrQualityDefinition {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrQualityDefinitionSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            quality_name: QualityName::Webdl1080p,
            title: None,
            min_size: Some(5.0),
            max_size: Some(50.0),
            preferred_size: Some(25.0),
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&qd))
        .await
        .expect("Failed to create SonarrQualityDefinition");

    // Update the resource
    let updated = SonarrQualityDefinition {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrQualityDefinitionSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            quality_name: QualityName::Webdl1080p,
            title: Some("WEB-DL 1080p Updated".to_string()),
            min_size: Some(10.0),
            max_size: Some(80.0),
            preferred_size: Some(40.0),
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&updated))
        .await
        .expect("Failed to update SonarrQualityDefinition");

    let retrieved = api
        .get(&name)
        .await
        .expect("Failed to get SonarrQualityDefinition");
    assert_eq!(
        retrieved.spec.title,
        Some("WEB-DL 1080p Updated".to_string())
    );
    assert_eq!(retrieved.spec.min_size, Some(10.0));
    assert_eq!(retrieved.spec.max_size, Some(80.0));

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test deleting a SonarrQualityDefinition resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_delete_sonarr_quality_definition() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("quality-def-delete");
    let api: Api<SonarrQualityDefinition> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    let qd = SonarrQualityDefinition {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrQualityDefinitionSpec {
            sonarr_instance_ref: SonarrInstanceRef {
                name: "test-sonarr".to_string(),
                namespace: None,
            },
            quality_name: QualityName::Hdtv720p,
            title: None,
            min_size: None,
            max_size: None,
            preferred_size: None,
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&qd))
        .await
        .expect("Failed to create SonarrQualityDefinition");

    let delete_result = api.delete(&name, &DeleteParams::default()).await;
    assert!(
        delete_result.is_ok(),
        "Failed to delete SonarrQualityDefinition: {:?}",
        delete_result.err()
    );

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let get_result = api.get(&name).await;
    assert!(
        get_result.is_err(),
        "SonarrQualityDefinition should have been deleted"
    );
}

/// Test configuring different quality levels
/// Note: Quality definitions already exist in Sonarr, the CRD is used to configure them
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_configure_different_quality_levels() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let api: Api<SonarrQualityDefinition> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    // Test configuring different quality levels - each maps to a pre-existing Sonarr quality
    let qualities = vec![
        ("sdtv", QualityName::Sdtv, 1.0, 10.0),
        ("hdtv720p", QualityName::Hdtv720p, 5.0, 50.0),
        ("webdl1080p", QualityName::Webdl1080p, 10.0, 80.0),
        ("bluray2160p", QualityName::Bluray2160p, 30.0, 200.0),
    ];

    let mut names = Vec::new();

    for (quality_slug, quality, min, max) in &qualities {
        let name = unique_name(&format!("quality-{}", quality_slug));
        names.push(name.clone());

        let qd = SonarrQualityDefinition {
            metadata: ObjectMeta {
                name: Some(name.clone()),
                namespace: Some(TEST_NAMESPACE.to_string()),
                ..Default::default()
            },
            spec: SonarrQualityDefinitionSpec {
                sonarr_instance_ref: SonarrInstanceRef {
                    name: "test-sonarr".to_string(),
                    namespace: None,
                },
                quality_name: quality.clone(),
                title: None,
                min_size: Some(*min),
                max_size: Some(*max),
                preferred_size: Some((min + max) / 2.0),
            },
            status: None,
        };

        api.patch(&name, &patch_params, &Patch::Apply(&qd))
            .await
            .expect(&format!(
                "Failed to create quality definition for {}",
                quality_slug
            ));
    }

    // Verify all were created
    for name in &names {
        let retrieved = api.get(name).await;
        assert!(
            retrieved.is_ok(),
            "Failed to get quality definition: {}",
            name
        );
    }

    // Cleanup
    for name in &names {
        let _ = api.delete(name, &DeleteParams::default()).await;
    }
}
