//! E2E tests for QualityProfile lifecycle

use crate::common::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, DeleteParams, Patch, PatchParams};
use sonarr_operator::crds::quality_profile::{Quality, QualityGroup};
use sonarr_operator::crds::{SonarrInstanceRef, SonarrQualityProfile, SonarrQualityProfileSpec};
use std::time::Duration;

/// Test the full lifecycle of a QualityProfile
#[tokio::test]
#[ignore = "requires E2E environment - run with: cargo test --test e2e -- --ignored"]
async fn test_quality_profile_full_lifecycle() {
    let mut ctx = TestContext::new()
        .await
        .expect("Failed to create test context");
    setup_e2e_namespace(&ctx.client)
        .await
        .expect("Failed to setup namespace");

    let qp_name = unique_name("e2e-qp");
    let profile_name = format!("E2E Test Profile {}", qp_name);

    ctx.register_cleanup("SonarrQualityProfile", E2E_NAMESPACE, &qp_name);

    // Create the quality profile CR with minimal required fields
    tracing::info!("Creating SonarrQualityProfile: {}", qp_name);
    let quality_profile = SonarrQualityProfile {
        metadata: ObjectMeta {
            name: Some(qp_name.clone()),
            namespace: Some(E2E_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrQualityProfileSpec {
            name: profile_name.clone(),
            upgrade_allowed: true,
            cutoff: 7, // Bluray-1080p
            quality_groups: vec![QualityGroup {
                id: None,
                name: Some("HD".to_string()),
                qualities: vec![Quality {
                    id: Some(7),
                    name: Some("Bluray-1080p".to_string()),
                    source: None,
                    resolution: None,
                }],
            }],
            sonarr_instance_ref: SonarrInstanceRef {
                name: "sonarr".to_string(),
                namespace: Some("default".to_string()),
            },
            cutoff_format_score: None,
            min_format_score: None,
            min_upgrade_format_score: None,
            format_items: vec![],
        },
        status: None,
    };

    apply_resource(&ctx.client, &quality_profile)
        .await
        .expect("Failed to create quality profile CR");

    // Wait for Ready condition
    tracing::info!("Waiting for quality profile to be ready...");
    let ready_qp =
        wait_for_ready::<SonarrQualityProfile>(&ctx.client, E2E_NAMESPACE, &qp_name, E2E_TIMEOUT)
            .await
            .expect("Quality profile never became ready");

    let qp_id = ready_qp
        .status
        .as_ref()
        .and_then(|s| s.id)
        .expect("Quality profile should have an ID");
    tracing::info!("Quality profile created with ID: {}", qp_id);

    // Verify in Sonarr API
    tracing::info!("Verifying quality profile exists in Sonarr...");
    let sonarr_qp = ctx
        .sonarr
        .find_quality_profile_by_name(&profile_name)
        .await
        .expect("Failed to query Sonarr API")
        .expect("Quality profile not found in Sonarr");

    assert_eq!(sonarr_qp.id, qp_id, "Quality profile ID mismatch");
    assert_eq!(
        sonarr_qp.name, profile_name,
        "Quality profile name mismatch"
    );
    assert!(sonarr_qp.upgrade_allowed, "Upgrade should be allowed");
    tracing::info!("✓ Quality profile verified in Sonarr");

    // Update the quality profile
    let updated_profile_name = format!("{} Updated", profile_name);
    tracing::info!("Updating quality profile name to: {}", updated_profile_name);

    let api: Api<SonarrQualityProfile> = Api::namespaced(ctx.client.clone(), E2E_NAMESPACE);
    let patch = serde_json::json!({
        "apiVersion": "devopsarr.io/v1alpha1",
        "kind": "SonarrQualityProfile",
        "metadata": {
            "name": qp_name,
            "namespace": E2E_NAMESPACE
        },
        "spec": {
            "name": updated_profile_name,
            "upgradeAllowed": false,
            "cutoff": 7,
            "qualityGroups": [{
                "name": "HD",
                "qualities": [{"id": 7, "name": "Bluray-1080p"}]
            }],
            "sonarrInstanceRef": {
                "name": "sonarr",
                "namespace": "default"
            }
        }
    });

    api.patch(
        &qp_name,
        &PatchParams::apply("sonarr-e2e-test").force(),
        &Patch::Apply(&patch),
    )
    .await
    .expect("Failed to update quality profile");

    tokio::time::sleep(Duration::from_secs(5)).await;

    // Verify update in Sonarr
    let updated_sonarr_qp = ctx
        .sonarr
        .find_quality_profile_by_name(&updated_profile_name)
        .await
        .expect("Failed to query Sonarr API")
        .expect("Updated quality profile not found in Sonarr");

    assert_eq!(updated_sonarr_qp.id, qp_id, "ID should not change");
    assert!(
        !updated_sonarr_qp.upgrade_allowed,
        "Upgrade should be disabled"
    );
    tracing::info!("✓ Quality profile update verified in Sonarr");

    // Delete and verify cleanup
    api.delete(&qp_name, &DeleteParams::default())
        .await
        .expect("Failed to delete quality profile CR");

    wait_for_deletion::<SonarrQualityProfile>(&ctx.client, E2E_NAMESPACE, &qp_name, QUICK_TIMEOUT)
        .await
        .expect("Quality profile CR was not deleted");

    tokio::time::sleep(Duration::from_secs(3)).await;

    let deleted = ctx
        .sonarr
        .find_quality_profile_by_name(&updated_profile_name)
        .await;
    assert!(
        matches!(deleted, Ok(None)),
        "Quality profile should be deleted from Sonarr"
    );
    tracing::info!("✓ Quality profile deleted from Sonarr");

    ctx.cleanup().await;
}
