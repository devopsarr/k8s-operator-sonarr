//! E2E tests for Tag lifecycle
//!
//! Tests the full lifecycle of creating, updating, and deleting tags
//! with verification against the Sonarr API.

use crate::common::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, DeleteParams, Patch, PatchParams};
use sonarr_operator::crds::{SonarrInstanceRef, SonarrTag, SonarrTagSpec};
use std::time::Duration;

/// Test the full lifecycle of a Tag: create -> verify in Sonarr -> update -> verify -> delete -> verify deleted
#[tokio::test]
#[ignore = "requires E2E environment - run with: cargo test --test e2e -- --ignored"]
async fn test_tag_full_lifecycle() {
    let mut ctx = TestContext::new()
        .await
        .expect("Failed to create test context");

    // Setup namespace
    setup_e2e_namespace(&ctx.client)
        .await
        .expect("Failed to setup namespace");

    let tag_name = unique_name("e2e-tag");
    let tag_label = format!("e2e-test-{}", &tag_name);

    // Register for cleanup
    ctx.register_cleanup("SonarrTag", E2E_NAMESPACE, &tag_name);

    // Step 1: Create the tag CR
    tracing::info!("Creating SonarrTag: {}", tag_name);
    let tag = SonarrTag {
        metadata: ObjectMeta {
            name: Some(tag_name.clone()),
            namespace: Some(E2E_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrTagSpec {
            label: tag_label.clone(),
            sonarr_instance_ref: SonarrInstanceRef {
                name: "sonarr".to_string(),
                namespace: Some("default".to_string()),
            },
        },
        status: None,
    };

    apply_resource(&ctx.client, &tag)
        .await
        .expect("Failed to create tag CR");

    // Step 2: Wait for Ready condition
    tracing::info!("Waiting for tag to be ready...");
    let ready_tag = wait_for_ready::<SonarrTag>(&ctx.client, E2E_NAMESPACE, &tag_name, E2E_TIMEOUT)
        .await
        .expect("Tag never became ready");

    // Verify the tag has an ID assigned
    let tag_id = ready_tag
        .status
        .as_ref()
        .and_then(|s| s.id)
        .expect("Tag should have an ID after reconciliation");
    tracing::info!("Tag created with ID: {}", tag_id);

    // Step 3: Verify in Sonarr API
    tracing::info!("Verifying tag exists in Sonarr...");
    let sonarr_tag = ctx
        .sonarr
        .find_tag_by_label(&tag_label)
        .await
        .expect("Failed to query Sonarr API")
        .expect("Tag not found in Sonarr");

    assert_eq!(sonarr_tag.id, tag_id, "Tag ID mismatch");
    assert_eq!(sonarr_tag.label, tag_label, "Tag label mismatch");
    tracing::info!("✓ Tag verified in Sonarr");

    // Step 4: Update the tag
    let updated_label = format!("{}-updated", tag_label);
    tracing::info!("Updating tag label to: {}", updated_label);

    let api: Api<SonarrTag> = Api::namespaced(ctx.client.clone(), E2E_NAMESPACE);
    let patch = serde_json::json!({
        "apiVersion": "devopsarr.io/v1alpha1",
        "kind": "SonarrTag",
        "metadata": {
            "name": tag_name,
            "namespace": E2E_NAMESPACE
        },
        "spec": {
            "label": updated_label,
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

    // Step 5: Verify update in Sonarr
    tracing::info!("Verifying tag update in Sonarr...");
    let updated_sonarr_tag = ctx
        .sonarr
        .find_tag_by_label(&updated_label)
        .await
        .expect("Failed to query Sonarr API")
        .expect("Updated tag not found in Sonarr");

    assert_eq!(
        updated_sonarr_tag.id, tag_id,
        "Tag ID should not change on update"
    );
    assert_eq!(
        updated_sonarr_tag.label, updated_label,
        "Tag label should be updated"
    );
    tracing::info!("✓ Tag update verified in Sonarr");

    // Step 6: Delete the tag CR
    tracing::info!("Deleting SonarrTag: {}", tag_name);
    api.delete(&tag_name, &DeleteParams::default())
        .await
        .expect("Failed to delete tag CR");

    // Step 7: Wait for deletion
    wait_for_deletion::<SonarrTag>(&ctx.client, E2E_NAMESPACE, &tag_name, QUICK_TIMEOUT)
        .await
        .expect("Tag CR was not deleted");

    // Step 8: Verify deleted from Sonarr (give operator time to process finalizer)
    tokio::time::sleep(Duration::from_secs(3)).await;

    tracing::info!("Verifying tag deleted from Sonarr...");
    let deleted_tag = ctx.sonarr.find_tag_by_label(&updated_label).await;

    match deleted_tag {
        Ok(None) => tracing::info!("✓ Tag successfully deleted from Sonarr"),
        Ok(Some(_)) => panic!("Tag still exists in Sonarr after CR deletion"),
        Err(e) => tracing::warn!("Could not verify deletion: {:?}", e),
    }

    // Cleanup is handled by TestContext
    ctx.cleanup().await;
}

/// Test creating multiple tags
#[tokio::test]
#[ignore = "requires E2E environment - run with: cargo test --test e2e -- --ignored"]
async fn test_multiple_tags() {
    let mut ctx = TestContext::new()
        .await
        .expect("Failed to create test context");
    setup_e2e_namespace(&ctx.client)
        .await
        .expect("Failed to setup namespace");

    let tag_names: Vec<String> = (1..=3)
        .map(|i| unique_name(&format!("e2e-multi-{}", i)))
        .collect();

    // Create multiple tags
    for tag_name in &tag_names {
        ctx.register_cleanup("SonarrTag", E2E_NAMESPACE, tag_name);

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
    }

    // Wait for all to be ready
    for tag_name in &tag_names {
        wait_for_ready::<SonarrTag>(&ctx.client, E2E_NAMESPACE, tag_name, E2E_TIMEOUT)
            .await
            .expect("Tag never became ready");
    }

    // Verify all exist in Sonarr
    for tag_name in &tag_names {
        let sonarr_tag = ctx
            .sonarr
            .find_tag_by_label(tag_name)
            .await
            .expect("Failed to query Sonarr")
            .expect("Tag not found in Sonarr");

        tracing::info!(
            "✓ Tag {} created in Sonarr with ID {}",
            tag_name,
            sonarr_tag.id
        );
    }

    ctx.cleanup().await;
}
