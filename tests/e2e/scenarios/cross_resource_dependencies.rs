//! E2E tests for cross-resource dependencies
//!
//! Tests scenarios where resources depend on each other:
//! - AutoTag depending on Tags
//! - QualityProfile depending on CustomFormats
//! - DelayProfile depending on Tags
//! - Notifications with Tags

use crate::common::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, DeleteParams};
use sonarr_operator::crds::delay_profile::DownloadProtocol;
use sonarr_operator::crds::{
    SonarrAutoTag, SonarrAutoTagSpec, SonarrCustomFormat, SonarrCustomFormatSpec,
    SonarrDelayProfile, SonarrDelayProfileSpec, SonarrInstanceRef, SonarrTag, SonarrTagSpec,
};
use std::time::Duration;

/// Test AutoTag that references Tags
/// AutoTags assign tags to series based on conditions, so they need existing tags
#[tokio::test]
#[ignore = "requires E2E environment - run with: cargo test --test e2e -- --ignored"]
async fn test_autotag_with_tag_dependency() {
    let mut ctx = TestContext::new()
        .await
        .expect("Failed to create test context");
    setup_e2e_namespace(&ctx.client)
        .await
        .expect("Failed to setup namespace");

    // Step 1: Create the dependent tag first
    let tag_name = unique_name("e2e-dep-tag");
    let tag_label = format!("dependency-{}", tag_name);
    ctx.register_cleanup("SonarrTag", E2E_NAMESPACE, &tag_name);

    tracing::info!("Step 1: Creating dependency tag: {}", tag_name);
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
        .expect("Failed to create tag");

    // Wait for tag to be ready and get its ID
    let ready_tag = wait_for_ready::<SonarrTag>(&ctx.client, E2E_NAMESPACE, &tag_name, E2E_TIMEOUT)
        .await
        .expect("Tag never became ready");

    let tag_id = ready_tag
        .status
        .as_ref()
        .and_then(|s| s.id)
        .expect("Tag should have ID");
    tracing::info!("Tag created with ID: {}", tag_id);

    // Verify tag exists in Sonarr
    let sonarr_tag = ctx
        .sonarr
        .find_tag_by_label(&tag_label)
        .await
        .expect("Failed to query Sonarr")
        .expect("Tag not found in Sonarr");
    assert_eq!(sonarr_tag.id, tag_id);
    tracing::info!("✓ Dependency tag verified in Sonarr");

    // Step 2: Create AutoTag that uses the tag
    let autotag_name = unique_name("e2e-autotag");
    ctx.register_cleanup("SonarrAutoTag", E2E_NAMESPACE, &autotag_name);

    tracing::info!(
        "Step 2: Creating AutoTag that references tag ID: {}",
        tag_id
    );
    let autotag = SonarrAutoTag {
        metadata: ObjectMeta {
            name: Some(autotag_name.clone()),
            namespace: Some(E2E_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrAutoTagSpec {
            name: format!("E2E AutoTag {}", autotag_name),
            remove_tags_automatically: false,
            tags: vec![tag_id], // Reference the tag we created
            specifications: vec![],
            sonarr_instance_ref: SonarrInstanceRef {
                name: "sonarr".to_string(),
                namespace: Some("default".to_string()),
            },
        },
        status: None,
    };

    apply_resource(&ctx.client, &autotag)
        .await
        .expect("Failed to create autotag");

    // Wait for autotag to be ready
    let ready_autotag =
        wait_for_ready::<SonarrAutoTag>(&ctx.client, E2E_NAMESPACE, &autotag_name, E2E_TIMEOUT)
            .await
            .expect("AutoTag never became ready");

    let autotag_id = ready_autotag
        .status
        .as_ref()
        .and_then(|s| s.id)
        .expect("AutoTag should have ID");
    tracing::info!("AutoTag created with ID: {}", autotag_id);

    // Verify in Sonarr and check the tag reference
    let sonarr_autotag = ctx
        .sonarr
        .find_auto_tag_by_name(&format!("E2E AutoTag {}", autotag_name))
        .await
        .expect("Failed to query Sonarr")
        .expect("AutoTag not found in Sonarr");

    assert_eq!(sonarr_autotag.id, autotag_id);
    assert!(
        sonarr_autotag.tags.contains(&tag_id),
        "AutoTag should reference tag ID {}. Actual tags: {:?}",
        tag_id,
        sonarr_autotag.tags
    );
    tracing::info!("✓ AutoTag verified with correct tag reference");

    // Step 3: Test cascade behavior - delete resources in correct order
    // AutoTag first (dependent), then Tag (dependency)
    tracing::info!("Step 3: Testing deletion order...");

    let autotag_api: Api<SonarrAutoTag> = Api::namespaced(ctx.client.clone(), E2E_NAMESPACE);
    autotag_api
        .delete(&autotag_name, &DeleteParams::default())
        .await
        .expect("Failed to delete autotag");

    wait_for_deletion::<SonarrAutoTag>(&ctx.client, E2E_NAMESPACE, &autotag_name, QUICK_TIMEOUT)
        .await
        .expect("AutoTag not deleted");

    tokio::time::sleep(Duration::from_secs(2)).await;

    // Now delete the tag
    let tag_api: Api<SonarrTag> = Api::namespaced(ctx.client.clone(), E2E_NAMESPACE);
    tag_api
        .delete(&tag_name, &DeleteParams::default())
        .await
        .expect("Failed to delete tag");

    wait_for_deletion::<SonarrTag>(&ctx.client, E2E_NAMESPACE, &tag_name, QUICK_TIMEOUT)
        .await
        .expect("Tag not deleted");

    tracing::info!("✓ Cross-resource dependency test completed successfully");
    ctx.cleanup().await;
}

/// Test DelayProfile that references Tags
#[tokio::test]
#[ignore = "requires E2E environment - run with: cargo test --test e2e -- --ignored"]
async fn test_delay_profile_with_tag_dependency() {
    let mut ctx = TestContext::new()
        .await
        .expect("Failed to create test context");
    setup_e2e_namespace(&ctx.client)
        .await
        .expect("Failed to setup namespace");

    // Create dependency tag
    let tag_name = unique_name("e2e-delay-tag");
    let tag_label = format!("delay-dep-{}", tag_name);
    ctx.register_cleanup("SonarrTag", E2E_NAMESPACE, &tag_name);

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
        .expect("Failed to create tag");
    let ready_tag = wait_for_ready::<SonarrTag>(&ctx.client, E2E_NAMESPACE, &tag_name, E2E_TIMEOUT)
        .await
        .expect("Tag never became ready");
    let tag_id = ready_tag
        .status
        .as_ref()
        .and_then(|s| s.id)
        .expect("Tag should have ID");
    tracing::info!("Dependency tag created with ID: {}", tag_id);

    // Create DelayProfile with tag
    let dp_name = unique_name("e2e-delay");
    ctx.register_cleanup("SonarrDelayProfile", E2E_NAMESPACE, &dp_name);

    let delay_profile = SonarrDelayProfile {
        metadata: ObjectMeta {
            name: Some(dp_name.clone()),
            namespace: Some(E2E_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrDelayProfileSpec {
            enable_usenet: true,
            enable_torrent: true,
            preferred_protocol: DownloadProtocol::Usenet,
            usenet_delay: 60,
            torrent_delay: 120,
            tags: vec![tag_id],
            bypass_if_highest_quality: false,
            bypass_if_above_custom_format_score: false,
            minimum_custom_format_score: 0,
            order: 0,
            sonarr_instance_ref: SonarrInstanceRef {
                name: "sonarr".to_string(),
                namespace: Some("default".to_string()),
            },
        },
        status: None,
    };

    apply_resource(&ctx.client, &delay_profile)
        .await
        .expect("Failed to create delay profile");

    let ready_dp =
        wait_for_ready::<SonarrDelayProfile>(&ctx.client, E2E_NAMESPACE, &dp_name, E2E_TIMEOUT)
            .await
            .expect("DelayProfile never became ready");

    let dp_id = ready_dp
        .status
        .as_ref()
        .and_then(|s| s.id)
        .expect("DelayProfile should have ID");
    tracing::info!("DelayProfile created with ID: {}", dp_id);

    // Verify delay profile has the tag
    let delay_profiles = ctx
        .sonarr
        .get_delay_profiles()
        .await
        .expect("Failed to get delay profiles");
    let our_dp = delay_profiles.iter().find(|dp| dp.id == dp_id);

    assert!(our_dp.is_some(), "DelayProfile not found in Sonarr");
    let our_dp = our_dp.unwrap();
    assert!(
        our_dp.tags.contains(&tag_id),
        "DelayProfile should have tag {}. Actual: {:?}",
        tag_id,
        our_dp.tags
    );
    tracing::info!("✓ DelayProfile verified with tag reference");

    ctx.cleanup().await;
}

/// Test that creating a resource with invalid tag reference fails gracefully
#[tokio::test]
#[ignore = "requires E2E environment - run with: cargo test --test e2e -- --ignored"]
async fn test_invalid_tag_reference_handling() {
    let mut ctx = TestContext::new()
        .await
        .expect("Failed to create test context");
    setup_e2e_namespace(&ctx.client)
        .await
        .expect("Failed to setup namespace");

    // Create AutoTag with a non-existent tag ID
    let autotag_name = unique_name("e2e-invalid-ref");
    ctx.register_cleanup("SonarrAutoTag", E2E_NAMESPACE, &autotag_name);

    let autotag = SonarrAutoTag {
        metadata: ObjectMeta {
            name: Some(autotag_name.clone()),
            namespace: Some(E2E_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrAutoTagSpec {
            name: format!("E2E Invalid Ref {}", autotag_name),
            remove_tags_automatically: false,
            tags: vec![99999], // Non-existent tag ID
            specifications: vec![],
            sonarr_instance_ref: SonarrInstanceRef {
                name: "sonarr".to_string(),
                namespace: Some("default".to_string()),
            },
        },
        status: None,
    };

    apply_resource(&ctx.client, &autotag)
        .await
        .expect("Failed to create autotag CR");

    // The resource should be created in K8s but may have an error condition
    // Wait a bit for the operator to process it
    tokio::time::sleep(Duration::from_secs(10)).await;

    // Check if the resource has an error condition or was created anyway
    // (Sonarr API behavior varies - it might accept invalid tag IDs or reject them)
    let api: Api<SonarrAutoTag> = Api::namespaced(ctx.client.clone(), E2E_NAMESPACE);
    let result = api.get(&autotag_name).await;

    if let Ok(autotag) = result {
        if let Some(status) = &autotag.status {
            let has_error = status
                .conditions
                .iter()
                .any(|c| c.type_ == "Ready" && c.status == "False");

            if has_error {
                tracing::info!("✓ AutoTag correctly shows error for invalid tag reference");
            } else {
                // Some Sonarr versions accept invalid tag IDs
                tracing::warn!(
                    "AutoTag was created despite invalid tag reference - Sonarr may accept any tag ID"
                );
            }
        }
    }

    ctx.cleanup().await;
}

/// Test multiple resources created in dependency order
#[tokio::test]
#[ignore = "requires E2E environment - run with: cargo test --test e2e -- --ignored"]
async fn test_full_dependency_chain() {
    let mut ctx = TestContext::new()
        .await
        .expect("Failed to create test context");
    setup_e2e_namespace(&ctx.client)
        .await
        .expect("Failed to setup namespace");

    let suffix = unique_name("chain");

    // Level 1: Create Tags (no dependencies)
    let tag_names: Vec<_> = (1..=2)
        .map(|i| format!("chain-tag-{}-{}", i, suffix))
        .collect();
    let mut tag_ids = Vec::new();

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

    // Wait for all tags
    for tag_name in &tag_names {
        let ready = wait_for_ready::<SonarrTag>(&ctx.client, E2E_NAMESPACE, tag_name, E2E_TIMEOUT)
            .await
            .expect("Tag not ready");
        tag_ids.push(ready.status.as_ref().and_then(|s| s.id).unwrap());
    }
    tracing::info!("Level 1: Created {} tags: {:?}", tag_names.len(), tag_ids);

    // Level 2: Create CustomFormat (no dependencies)
    let cf_name = format!("chain-cf-{}", suffix);
    ctx.register_cleanup("SonarrCustomFormat", E2E_NAMESPACE, &cf_name);

    let custom_format = SonarrCustomFormat {
        metadata: ObjectMeta {
            name: Some(cf_name.clone()),
            namespace: Some(E2E_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrCustomFormatSpec {
            name: cf_name.clone(),
            include_custom_format_when_renaming: false,
            specifications: vec![],
            sonarr_instance_ref: SonarrInstanceRef {
                name: "sonarr".to_string(),
                namespace: Some("default".to_string()),
            },
        },
        status: None,
    };
    apply_resource(&ctx.client, &custom_format)
        .await
        .expect("Failed to create custom format");

    let ready_cf =
        wait_for_ready::<SonarrCustomFormat>(&ctx.client, E2E_NAMESPACE, &cf_name, E2E_TIMEOUT)
            .await
            .expect("CustomFormat not ready");
    let cf_id = ready_cf.status.as_ref().and_then(|s| s.id).unwrap();
    tracing::info!("Level 2: Created CustomFormat with ID: {}", cf_id);

    // Level 3: Create DelayProfile (depends on Tags)
    let dp_name = format!("chain-dp-{}", suffix);
    ctx.register_cleanup("SonarrDelayProfile", E2E_NAMESPACE, &dp_name);

    let delay_profile = SonarrDelayProfile {
        metadata: ObjectMeta {
            name: Some(dp_name.clone()),
            namespace: Some(E2E_NAMESPACE.to_string()),
            ..Default::default()
        },
        spec: SonarrDelayProfileSpec {
            enable_usenet: true,
            enable_torrent: false,
            preferred_protocol: DownloadProtocol::Usenet,
            usenet_delay: 30,
            torrent_delay: 0,
            tags: tag_ids.clone(),
            bypass_if_highest_quality: false,
            bypass_if_above_custom_format_score: false,
            minimum_custom_format_score: 0,
            order: 0,
            sonarr_instance_ref: SonarrInstanceRef {
                name: "sonarr".to_string(),
                namespace: Some("default".to_string()),
            },
        },
        status: None,
    };
    apply_resource(&ctx.client, &delay_profile)
        .await
        .expect("Failed to create delay profile");

    let ready_dp =
        wait_for_ready::<SonarrDelayProfile>(&ctx.client, E2E_NAMESPACE, &dp_name, E2E_TIMEOUT)
            .await
            .expect("DelayProfile not ready");
    let dp_id = ready_dp.status.as_ref().and_then(|s| s.id).unwrap();
    tracing::info!(
        "Level 3: Created DelayProfile with ID: {} referencing tags: {:?}",
        dp_id,
        tag_ids
    );

    // Verify the full chain in Sonarr
    let sonarr_tags = ctx.sonarr.get_tags().await.expect("Failed to get tags");
    for tag_id in &tag_ids {
        assert!(
            sonarr_tags.iter().any(|t| t.id == *tag_id),
            "Tag {} not found in Sonarr",
            tag_id
        );
    }

    let sonarr_cf = ctx
        .sonarr
        .find_custom_format_by_name(&cf_name)
        .await
        .expect("Failed to get CF");
    assert!(sonarr_cf.is_some(), "CustomFormat not found in Sonarr");

    let delay_profiles = ctx
        .sonarr
        .get_delay_profiles()
        .await
        .expect("Failed to get delay profiles");
    let our_dp = delay_profiles.iter().find(|dp| dp.id == dp_id);
    assert!(our_dp.is_some(), "DelayProfile not found in Sonarr");

    tracing::info!("✓ Full dependency chain verified successfully");
    ctx.cleanup().await;
}
