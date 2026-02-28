use std::sync::Arc;

use kube::api::{Api, Patch, PatchParams};
use kube::runtime::controller::Action;
use kube::{Client, ResourceExt};
use tracing::info;

use sonarr::apis::{custom_format_api, quality_definition_api, quality_profile_api};
use sonarr::models::{
    ProfileFormatItemResource, Quality as SonarrQuality, QualityProfileQualityItemResource,
    QualityProfileResource,
};

use crate::Context;
use crate::crds::{SonarrQualityProfile, SonarrQualityProfileStatus};
use crate::error::{Error, Result};

use super::tag::get_sonarr_config;
use super::traits::{REQUEUE_DURATION, reconcile_with_finalizer, run_controller};
use super::{ready_condition, update_conditions};

/// Start the SonarrQualityProfile controller
pub async fn run(client: Client, context: Arc<Context>) {
    run_controller::<SonarrQualityProfile, _, _>(
        client,
        context,
        "SonarrQualityProfile",
        reconcile,
    )
    .await;
}

async fn reconcile(obj: Arc<SonarrQualityProfile>, ctx: Arc<Context>) -> Result<Action> {
    reconcile_with_finalizer(obj, ctx, reconcile_apply, reconcile_cleanup).await
}

async fn reconcile_apply(profile: Arc<SonarrQualityProfile>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = profile
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;
    let name = profile.name_any();

    info!("Reconciling SonarrQualityProfile: {}/{}", namespace, name);

    let config =
        get_sonarr_config(&ctx, client, &namespace, &profile.spec.sonarr_instance_ref).await?;

    // Fetch all quality definitions and custom formats from Sonarr
    // (following the Terraform provider approach: include ALL qualities and formats)
    let all_quality_defs = quality_definition_api::list_quality_definition(&config).await?;
    let all_custom_formats = custom_format_api::list_custom_format(&config).await?;

    // Build quality profile resource
    let mut qp_resource = QualityProfileResource::new();
    qp_resource.name = Some(Some(profile.spec.name.clone()));
    qp_resource.upgrade_allowed = Some(profile.spec.upgrade_allowed);
    qp_resource.cutoff = Some(profile.spec.cutoff);
    qp_resource.cutoff_format_score = profile.spec.cutoff_format_score;
    qp_resource.min_format_score = profile.spec.min_format_score;
    qp_resource.min_upgrade_format_score = Some(profile.spec.min_upgrade_format_score.unwrap_or(1));

    // Build full items list: allowed qualities from spec + not-allowed for the rest
    qp_resource.items = Some(Some(build_quality_items(
        &profile.spec.quality_groups,
        &all_quality_defs,
    )));

    // Build full format items list: scored items from spec + score=0 for the rest
    qp_resource.format_items = Some(Some(build_format_items(
        &profile.spec.format_items,
        &all_custom_formats,
    )));

    let sonarr_profile = if let Some(id) = profile.status.as_ref().and_then(|s| s.id) {
        qp_resource.id = Some(id);
        match quality_profile_api::update_quality_profile(
            &config,
            id.to_string().as_str(),
            Some(qp_resource.clone()),
        )
        .await
        {
            Ok(p) => p,
            Err(_) => {
                qp_resource.id = None;
                quality_profile_api::create_quality_profile(&config, Some(qp_resource)).await?
            }
        }
    } else {
        let existing = quality_profile_api::list_quality_profile(&config).await?;
        if let Some(existing_profile) = existing
            .iter()
            .find(|p| p.name.as_ref().and_then(|n| n.as_ref()) == Some(&profile.spec.name))
        {
            existing_profile.clone()
        } else {
            quality_profile_api::create_quality_profile(&config, Some(qp_resource)).await?
        }
    };

    // Update status
    let profiles_api: Api<SonarrQualityProfile> = Api::namespaced(client.clone(), &namespace);
    let mut conditions = profile
        .status
        .as_ref()
        .map(|s| s.conditions.clone())
        .unwrap_or_default();
    update_conditions(
        &mut conditions,
        ready_condition(true, "Synced", "Quality profile synchronized with Sonarr"),
    );

    let status = SonarrQualityProfileStatus {
        conditions,
        id: sonarr_profile.id,
        observed_generation: profile.metadata.generation.unwrap_or(0),
    };

    let status_patch = serde_json::json!({ "status": status });
    profiles_api
        .patch_status(&name, &PatchParams::default(), &Patch::Merge(&status_patch))
        .await?;

    Ok(Action::requeue(REQUEUE_DURATION))
}

/// Build the full quality items list following the Terraform provider approach:
/// 1. Convert user-specified quality groups to allowed items
/// 2. Fill remaining quality definitions as not-allowed
/// 3. Reverse the list (higher quality to lower)
fn build_quality_items(
    groups: &[crate::crds::quality_profile::QualityGroup],
    all_quality_defs: &[sonarr::models::QualityDefinitionResource],
) -> Vec<QualityProfileQualityItemResource> {
    let mut allowed_quality_ids: Vec<i32> = Vec::new();
    let mut items: Vec<QualityProfileQualityItemResource> = Vec::new();

    // Convert user-specified quality groups to allowed items
    for group in groups {
        if group.qualities.len() == 1 {
            // Single quality — set quality field directly (like Terraform provider)
            let q = &group.qualities[0];
            let mut quality = SonarrQuality::new();
            quality.id = q.id;
            quality.name = q.name.clone().map(Some);

            let mut item = QualityProfileQualityItemResource::new();
            item.allowed = Some(true);
            item.quality = Some(Box::new(quality));
            // Single quality items don't set `items` (Terraform provider doesn't call SetItems)

            if let Some(id) = q.id {
                allowed_quality_ids.push(id);
            }
            items.push(item);
        } else {
            // Quality group with multiple nested qualities
            let sub_items: Vec<QualityProfileQualityItemResource> = group
                .qualities
                .iter()
                .map(|q| {
                    let mut quality = SonarrQuality::new();
                    quality.id = q.id;
                    quality.name = q.name.clone().map(Some);

                    if let Some(id) = q.id {
                        allowed_quality_ids.push(id);
                    }

                    let mut sub_item = QualityProfileQualityItemResource::new();
                    sub_item.allowed = Some(true);
                    sub_item.quality = Some(Box::new(quality));
                    sub_item
                })
                .collect();

            let mut item = QualityProfileQualityItemResource::new();
            item.id = group.id;
            item.name = group.name.clone().map(Some);
            item.allowed = Some(true);
            item.items = Some(Some(sub_items));
            items.push(item);
        }
    }

    // Fill remaining quality definitions as not-allowed
    for qd in all_quality_defs {
        let quality_id = qd.quality.as_ref().and_then(|q| q.id);
        if let Some(id) = quality_id
            && !allowed_quality_ids.contains(&id)
        {
            let mut quality = SonarrQuality::new();
            quality.id = Some(id);

            let mut item = QualityProfileQualityItemResource::new();
            item.allowed = Some(false);
            item.items = Some(Some(vec![]));
            item.quality = Some(Box::new(quality));
            items.push(item);
        }
    }

    // Reverse: higher quality to lower (matches Terraform provider behavior)
    items.reverse();
    items
}

/// Build the full format items list following the Terraform provider approach:
/// 1. Include user-specified format items with their scores
/// 2. Fill remaining custom formats with score=0
fn build_format_items(
    spec_items: &[crate::crds::quality_profile::FormatItem],
    all_custom_formats: &[sonarr::models::CustomFormatResource],
) -> Vec<ProfileFormatItemResource> {
    let mut used_format_ids: Vec<i32> = Vec::new();
    let mut items: Vec<ProfileFormatItemResource> = Vec::new();

    // Convert user-specified format items
    for fi in spec_items {
        let mut item = ProfileFormatItemResource::new();
        item.format = fi.format;
        item.name = fi.name.clone().map(Some);
        item.score = Some(fi.score);
        if let Some(id) = fi.format {
            used_format_ids.push(id);
        }
        items.push(item);
    }

    // Fill remaining custom formats with score=0
    for cf in all_custom_formats {
        if let Some(id) = cf.id
            && !used_format_ids.contains(&id)
        {
            let mut item = ProfileFormatItemResource::new();
            item.format = Some(id);
            item.score = Some(0);
            items.push(item);
        }
    }

    items
}

async fn reconcile_cleanup(
    profile: Arc<SonarrQualityProfile>,
    ctx: Arc<Context>,
) -> Result<Action> {
    let client = &ctx.client;
    let namespace = profile
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;

    info!(
        "Cleaning up SonarrQualityProfile: {}/{}",
        namespace,
        profile.name_any()
    );

    if let Some(id) = profile.status.as_ref().and_then(|s| s.id)
        && let Ok(config) =
            get_sonarr_config(&ctx, client, &namespace, &profile.spec.sonarr_instance_ref).await
    {
        let _ = quality_profile_api::delete_quality_profile(&config, id).await;
    }

    Ok(Action::await_change())
}
