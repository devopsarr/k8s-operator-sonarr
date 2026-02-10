//! Integration tests for the Sonarr CRD
//!
//! These tests verify that Sonarr CRDs can be created, read, updated, and deleted
//! in a real Kubernetes cluster.

use crate::common::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, DeleteParams, Patch, PatchParams};
use sonarr_operator::crds::{ServiceConfig, Sonarr, SonarrSpec, StorageConfig};

/// Test that the Sonarr CRD is installed and established
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_sonarr_crd_is_established() {
    let client = test_client().await;

    assert!(
        is_crd_established(&client, "sonarrs.devopsarr.io").await,
        "Sonarr CRD is not established - run 'make install' first"
    );
}

/// Test creating a minimal Sonarr resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_create_minimal_sonarr() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("sonarr-minimal");
    let sonarr = Sonarr {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrSpec::default(),
        status: None,
    };

    // Create the resource
    let api: Api<Sonarr> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();
    let result = api
        .patch(&name, &patch_params, &Patch::Apply(&sonarr))
        .await;

    assert!(
        result.is_ok(),
        "Failed to create Sonarr: {:?}",
        result.err()
    );

    // Verify it exists
    let retrieved = api.get(&name).await;
    assert!(
        retrieved.is_ok(),
        "Failed to get Sonarr: {:?}",
        retrieved.err()
    );

    let sonarr = retrieved.unwrap();
    assert_eq!(sonarr.spec.image, "lscr.io/linuxserver/sonarr:latest");
    assert_eq!(sonarr.spec.replicas, 1);

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test creating a Sonarr resource with custom configuration
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_create_sonarr_with_custom_config() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("sonarr-custom");
    let sonarr = Sonarr {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrSpec {
            image: "lscr.io/linuxserver/sonarr:develop".to_string(),
            replicas: 1,
            service: ServiceConfig {
                port: 9090,
                container_port: 8989,
                service_type: "ClusterIP".to_string(),
                ..Default::default()
            },
            storage: StorageConfig {
                size: "5Gi".to_string(),
                ..Default::default()
            },
            ..Default::default()
        },
        status: None,
    };

    // Create the resource
    let api: Api<Sonarr> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();
    let result = api
        .patch(&name, &patch_params, &Patch::Apply(&sonarr))
        .await;

    assert!(
        result.is_ok(),
        "Failed to create Sonarr: {:?}",
        result.err()
    );

    // Verify custom values
    let retrieved = api.get(&name).await.expect("Failed to get Sonarr");
    assert_eq!(retrieved.spec.image, "lscr.io/linuxserver/sonarr:develop");
    assert_eq!(retrieved.spec.service.port, 9090);
    assert_eq!(retrieved.spec.storage.size, "5Gi");

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test updating a Sonarr resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_update_sonarr() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("sonarr-update");
    let api: Api<Sonarr> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    // Create initial resource
    let sonarr = Sonarr {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrSpec {
            image: "lscr.io/linuxserver/sonarr:latest".to_string(),
            ..Default::default()
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&sonarr))
        .await
        .expect("Failed to create Sonarr");

    // Update the resource
    let updated_sonarr = Sonarr {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrSpec {
            image: "lscr.io/linuxserver/sonarr:develop".to_string(),
            ..Default::default()
        },
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&updated_sonarr))
        .await
        .expect("Failed to update Sonarr");

    // Verify the update
    let retrieved = api.get(&name).await.expect("Failed to get Sonarr");
    assert_eq!(retrieved.spec.image, "lscr.io/linuxserver/sonarr:develop");

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test deleting a Sonarr resource
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_delete_sonarr() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("sonarr-delete");
    let api: Api<Sonarr> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    // Create resource
    let sonarr = Sonarr {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrSpec::default(),
        status: None,
    };

    api.patch(&name, &patch_params, &Patch::Apply(&sonarr))
        .await
        .expect("Failed to create Sonarr");

    // Delete the resource
    let delete_result = api.delete(&name, &DeleteParams::default()).await;
    assert!(
        delete_result.is_ok(),
        "Failed to delete Sonarr: {:?}",
        delete_result.err()
    );

    // Verify it's deleted (may take a moment)
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let get_result = api.get(&name).await;
    assert!(get_result.is_err(), "Sonarr should have been deleted");
}

/// Test listing Sonarr resources
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_list_sonarrs() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let api: Api<Sonarr> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();

    // Create a few resources
    let names: Vec<String> = (0..3)
        .map(|i| unique_name(&format!("sonarr-list-{}", i)))
        .collect();

    for name in &names {
        let sonarr = Sonarr {
            metadata: ObjectMeta {
                name: Some(name.clone()),
                namespace: Some(TEST_NAMESPACE.to_string()),
                labels: Some([("test-group".to_string(), "list-test".to_string())].into()),
                ..Default::default()
            },
            spec: SonarrSpec::default(),
            status: None,
        };
        api.patch(name, &patch_params, &Patch::Apply(&sonarr))
            .await
            .expect("Failed to create Sonarr");
    }

    // List with label selector
    let lp = kube::api::ListParams::default().labels("test-group=list-test");
    let list = api.list(&lp).await.expect("Failed to list Sonarrs");

    assert!(
        list.items.len() >= 3,
        "Expected at least 3 Sonarrs, got {}",
        list.items.len()
    );

    // Cleanup
    for name in &names {
        let _ = api.delete(name, &DeleteParams::default()).await;
    }
}

/// Test that validation rejects invalid Sonarr specs
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_sonarr_with_ingress_config() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("sonarr-ingress");
    let sonarr = Sonarr {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrSpec {
            ingress: Some(sonarr_operator::crds::sonarr::IngressConfig {
                enabled: true,
                host: "sonarr.example.com".to_string(),
                path: "/".to_string(),
                path_type: "Prefix".to_string(),
                ingress_class_name: Some("nginx".to_string()),
                tls: None,
                annotations: Default::default(),
            }),
            ..Default::default()
        },
        status: None,
    };

    let api: Api<Sonarr> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();
    let result = api
        .patch(&name, &patch_params, &Patch::Apply(&sonarr))
        .await;

    assert!(
        result.is_ok(),
        "Failed to create Sonarr with ingress config: {:?}",
        result.err()
    );

    // Verify ingress config
    let retrieved = api.get(&name).await.expect("Failed to get Sonarr");
    let ingress = retrieved.spec.ingress.expect("Ingress config should exist");
    assert!(ingress.enabled);
    assert_eq!(ingress.host, "sonarr.example.com");

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}

/// Test Sonarr with HTTPRoute configuration for Gateway API
#[tokio::test]
#[ignore = "requires kubernetes cluster - run with: cargo test --test integration -- --ignored"]
async fn test_sonarr_with_http_route_config() {
    let client = test_client().await;
    ensure_test_namespace(&client)
        .await
        .expect("Failed to create test namespace");

    let name = unique_name("sonarr-httproute");
    let sonarr = Sonarr {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(TEST_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrSpec {
            http_route: Some(sonarr_operator::crds::sonarr::HTTPRouteConfig {
                enabled: true,
                gateway_ref: sonarr_operator::crds::sonarr::GatewayRef {
                    name: "my-gateway".to_string(),
                    namespace: Some("gateway-system".to_string()),
                    section_name: None,
                },
                hostnames: vec!["sonarr.example.com".to_string()],
                path: "/".to_string(),
                path_type: "PathPrefix".to_string(),
                labels: Default::default(),
                annotations: Default::default(),
            }),
            ..Default::default()
        },
        status: None,
    };

    let api: Api<Sonarr> = Api::namespaced(client.clone(), TEST_NAMESPACE);
    let patch_params = PatchParams::apply("sonarr-operator-test").force();
    let result = api
        .patch(&name, &patch_params, &Patch::Apply(&sonarr))
        .await;

    assert!(
        result.is_ok(),
        "Failed to create Sonarr with HTTPRoute config: {:?}",
        result.err()
    );

    // Verify HTTPRoute config
    let retrieved = api.get(&name).await.expect("Failed to get Sonarr");
    let http_route = retrieved
        .spec
        .http_route
        .expect("HTTPRoute config should exist");
    assert!(http_route.enabled);
    assert_eq!(http_route.gateway_ref.name, "my-gateway");
    assert_eq!(http_route.hostnames, vec!["sonarr.example.com".to_string()]);

    // Cleanup
    let _ = api.delete(&name, &DeleteParams::default()).await;
}
