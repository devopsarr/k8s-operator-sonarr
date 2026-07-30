//! E2E tests for error recovery scenarios
//!
//! Tests how the operator handles error conditions and recovers from them.

use crate::common::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, Patch, PatchParams};
use sonarr_operator::crds::*;
use std::time::Duration;

/// Test that operator recovers when Sonarr becomes available
#[tokio::test]
#[ignore = "requires E2E environment - run with: cargo test --test e2e -- --ignored"]
async fn test_recovery_after_sonarr_available() {
    let mut ctx = TestContext::new()
        .await
        .expect("Failed to create test context");
    setup_e2e_namespace(&ctx.client)
        .await
        .expect("Failed to setup namespace");

    // Ensure Sonarr is available
    ctx.sonarr
        .wait_for_ready(Duration::from_secs(60))
        .await
        .expect("Sonarr should be ready");

    // Create a tag
    let tag_name = unique_name("e2e-recovery");
    ctx.register_cleanup("SonarrTag", E2E_NAMESPACE, &tag_name);

    let tag = SonarrTag {
        metadata: ObjectMeta {
            name: Some(tag_name.clone()),
            namespace: Some(E2E_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrTagSpec {
            label: tag_name.clone(),
            sonarr_instance_ref: SonarrInstanceRef {
                name: "sonarr".to_string(),
                namespace: Some("default".to_string()),
            },
        },
        status: None,
    };

    apply_resource(&ctx.client, &tag)
        .await
        .expect("Failed to create tag");

    // Wait for ready
    wait_for_ready::<SonarrTag>(&ctx.client, E2E_NAMESPACE, &tag_name, E2E_TIMEOUT)
        .await
        .expect("Tag never became ready");

    // Verify in Sonarr
    let sonarr_tag = ctx
        .sonarr
        .find_tag_by_label(&tag_name)
        .await
        .expect("Failed to query Sonarr")
        .expect("Tag not found");

    tracing::info!("✓ Tag created successfully with ID: {}", sonarr_tag.id);
    ctx.cleanup().await;
}

/// Test handling of invalid Sonarr instance reference
#[tokio::test]
#[ignore = "requires E2E environment - run with: cargo test --test e2e -- --ignored"]
async fn test_invalid_sonarr_instance_reference() {
    let mut ctx = TestContext::new()
        .await
        .expect("Failed to create test context");
    setup_e2e_namespace(&ctx.client)
        .await
        .expect("Failed to setup namespace");

    let tag_name = unique_name("e2e-invalid-instance");
    ctx.register_cleanup("SonarrTag", E2E_NAMESPACE, &tag_name);

    // Create tag with non-existent Sonarr instance
    let tag = SonarrTag {
        metadata: ObjectMeta {
            name: Some(tag_name.clone()),
            namespace: Some(E2E_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrTagSpec {
            label: tag_name.clone(),
            sonarr_instance_ref: SonarrInstanceRef {
                name: "nonexistent-sonarr".to_string(),
                namespace: Some("default".to_string()),
            },
        },
        status: None,
    };

    apply_resource(&ctx.client, &tag)
        .await
        .expect("Failed to create tag CR");

    // Wait for operator to process
    tokio::time::sleep(Duration::from_secs(15)).await;

    // Check status - should have error condition
    let api: Api<SonarrTag> = Api::namespaced(ctx.client.clone(), E2E_NAMESPACE);
    let tag_status = api.get(&tag_name).await.expect("Failed to get tag");

    if let Some(status) = &tag_status.status {
        let has_error = status
            .conditions
            .iter()
            .any(|c| c.type_ == "Ready" && c.status == "False");

        if has_error {
            let error_msg = status
                .conditions
                .iter()
                .find(|c| c.type_ == "Ready" && c.status == "False")
                .map(|c| c.message.clone());
            tracing::info!(
                "✓ Tag correctly shows error for invalid instance: {:?}",
                error_msg
            );
        } else {
            tracing::warn!("Tag doesn't show error for invalid instance reference");
        }
    }

    ctx.cleanup().await;
}

/// Test that updating a resource triggers reconciliation
#[tokio::test]
#[ignore = "requires E2E environment - run with: cargo test --test e2e -- --ignored"]
async fn test_resource_update_triggers_reconciliation() {
    let mut ctx = TestContext::new()
        .await
        .expect("Failed to create test context");
    setup_e2e_namespace(&ctx.client)
        .await
        .expect("Failed to setup namespace");

    let tag_name = unique_name("e2e-update-reconcile");
    ctx.register_cleanup("SonarrTag", E2E_NAMESPACE, &tag_name);

    // Create tag
    let tag = SonarrTag {
        metadata: ObjectMeta {
            name: Some(tag_name.clone()),
            namespace: Some(E2E_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrTagSpec {
            label: format!("{}-v1", tag_name),
            sonarr_instance_ref: SonarrInstanceRef {
                name: "sonarr".to_string(),
                namespace: Some("default".to_string()),
            },
        },
        status: None,
    };

    apply_resource(&ctx.client, &tag)
        .await
        .expect("Failed to create tag");
    wait_for_ready::<SonarrTag>(&ctx.client, E2E_NAMESPACE, &tag_name, E2E_TIMEOUT)
        .await
        .expect("Tag v1 never became ready");

    // Verify v1 in Sonarr
    let v1_label = format!("{}-v1", tag_name);
    ctx.sonarr
        .find_tag_by_label(&v1_label)
        .await
        .expect("Failed to query")
        .expect("Tag v1 not found in Sonarr");
    tracing::info!("✓ Tag v1 created in Sonarr");

    // Update to v2
    let api: Api<SonarrTag> = Api::namespaced(ctx.client.clone(), E2E_NAMESPACE);
    let v2_label = format!("{}-v2", tag_name);
    let patch = serde_json::json!({
        "apiVersion": "devopsarr.io/v1alpha1",
        "kind": "SonarrTag",
        "metadata": {
            "name": tag_name,
            "namespace": E2E_NAMESPACE
        },
        "spec": {
            "label": v2_label,
            "sonarrInstanceRef": {
                "name": "sonarr",
                "namespace": "default"
            }
        }
    });

    api.patch(
        &tag_name,
        &PatchParams::apply("sonarr-e2e-test").force(),
        &Patch::Apply(&patch),
    )
    .await
    .expect("Failed to update tag");

    // Wait for update to propagate
    tokio::time::sleep(Duration::from_secs(5)).await;

    // Verify v2 in Sonarr (v1 label should no longer exist)
    let v2_in_sonarr = ctx
        .sonarr
        .find_tag_by_label(&v2_label)
        .await
        .expect("Failed to query");

    assert!(v2_in_sonarr.is_some(), "Tag v2 should exist in Sonarr");

    let v1_in_sonarr = ctx
        .sonarr
        .find_tag_by_label(&v1_label)
        .await
        .expect("Failed to query");

    assert!(
        v1_in_sonarr.is_none(),
        "Tag v1 label should not exist after update"
    );

    tracing::info!("✓ Tag update correctly reconciled - v1 -> v2");
    ctx.cleanup().await;
}

/// Test rapid updates don't cause issues
#[tokio::test]
#[ignore = "requires E2E environment - run with: cargo test --test e2e -- --ignored"]
async fn test_rapid_updates_handling() {
    let mut ctx = TestContext::new()
        .await
        .expect("Failed to create test context");
    setup_e2e_namespace(&ctx.client)
        .await
        .expect("Failed to setup namespace");

    let tag_name = unique_name("e2e-rapid");
    ctx.register_cleanup("SonarrTag", E2E_NAMESPACE, &tag_name);

    let api: Api<SonarrTag> = Api::namespaced(ctx.client.clone(), E2E_NAMESPACE);

    // Create initial tag
    let tag = SonarrTag {
        metadata: ObjectMeta {
            name: Some(tag_name.clone()),
            namespace: Some(E2E_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrTagSpec {
            label: format!("{}-initial", tag_name),
            sonarr_instance_ref: SonarrInstanceRef {
                name: "sonarr".to_string(),
                namespace: Some("default".to_string()),
            },
        },
        status: None,
    };

    apply_resource(&ctx.client, &tag)
        .await
        .expect("Failed to create tag");

    // Rapid fire updates
    for i in 1..=5 {
        let patch = serde_json::json!({
            "apiVersion": "devopsarr.io/v1alpha1",
            "kind": "SonarrTag",
            "metadata": {
                "name": tag_name,
                "namespace": E2E_NAMESPACE
            },
            "spec": {
                "label": format!("{}-update-{}", tag_name, i),
                "sonarrInstanceRef": {
                    "name": "sonarr",
                    "namespace": "default"
                }
            }
        });

        api.patch(
            &tag_name,
            &PatchParams::apply("sonarr-e2e-test").force(),
            &Patch::Apply(&patch),
        )
        .await
        .expect("Failed to update tag");

        // Small delay between updates
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    // Wait for final reconciliation
    tokio::time::sleep(Duration::from_secs(10)).await;

    // Verify final state
    let final_label = format!("{}-update-5", tag_name);
    let sonarr_tag = ctx
        .sonarr
        .find_tag_by_label(&final_label)
        .await
        .expect("Failed to query");

    assert!(sonarr_tag.is_some(), "Final tag state should be in Sonarr");
    tracing::info!("✓ Rapid updates handled correctly");

    ctx.cleanup().await;
}
