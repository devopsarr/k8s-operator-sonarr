use std::sync::Arc;

use kube::api::{Api, Patch, PatchParams};
use kube::runtime::controller::Action;
use kube::{Client, ResourceExt};
use tracing::info;

use sonarr::apis::language_profile_api;
use sonarr::models::{Language, LanguageProfileItemResource, LanguageProfileResource};

use crate::Context;
use crate::crds::language_profile::LanguageType;
use crate::crds::{SonarrLanguageProfile, SonarrLanguageProfileStatus};
use crate::error::{Error, Result};

use super::tag::get_sonarr_config;
use super::traits::{REQUEUE_DURATION, reconcile_with_finalizer, run_controller};
use super::{ready_condition, update_conditions};

/// Start the SonarrLanguageProfile controller
pub async fn run(client: Client, context: Arc<Context>) {
    run_controller::<SonarrLanguageProfile, _, _>(
        client,
        context,
        "SonarrLanguageProfile",
        reconcile,
    )
    .await;
}

async fn reconcile(obj: Arc<SonarrLanguageProfile>, ctx: Arc<Context>) -> Result<Action> {
    reconcile_with_finalizer(obj, ctx, reconcile_apply, reconcile_cleanup).await
}

async fn reconcile_apply(profile: Arc<SonarrLanguageProfile>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = profile
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;
    let name = profile.name_any();

    info!("Reconciling SonarrLanguageProfile: {}/{}", namespace, name);

    let config =
        get_sonarr_config(&ctx, client, &namespace, &profile.spec.sonarr_instance_ref).await?;

    // Build language profile resource
    let mut resource = LanguageProfileResource::new();
    resource.name = Some(Some(profile.spec.name.clone()));
    resource.upgrade_allowed = Some(profile.spec.upgrade_allowed);
    resource.cutoff = Some(Box::new(convert_language(&profile.spec.cutoff_language)));

    // Convert languages
    let languages: Vec<LanguageProfileItemResource> = profile
        .spec
        .languages
        .iter()
        .map(|item| {
            let mut lang_item = LanguageProfileItemResource::new();
            lang_item.language = Some(Box::new(convert_language(&item.language)));
            lang_item.allowed = Some(item.allowed);
            lang_item
        })
        .collect();
    resource.languages = Some(Some(languages));

    let sonarr_profile = if let Some(id) = profile.status.as_ref().and_then(|s| s.id) {
        resource.id = Some(id);
        match language_profile_api::update_language_profile(
            &config,
            &id.to_string(),
            Some(resource.clone()),
        )
        .await
        {
            Ok(p) => p,
            Err(_) => {
                resource.id = None;
                language_profile_api::create_language_profile(&config, Some(resource)).await?
            }
        }
    } else {
        let existing = language_profile_api::list_language_profile(&config).await?;
        if let Some(existing_item) = existing
            .iter()
            .find(|p| p.name.as_ref().and_then(|n| n.as_ref()) == Some(&profile.spec.name))
        {
            existing_item.clone()
        } else {
            language_profile_api::create_language_profile(&config, Some(resource)).await?
        }
    };

    // Update status
    let api: Api<SonarrLanguageProfile> = Api::namespaced(client.clone(), &namespace);
    let mut conditions = profile
        .status
        .as_ref()
        .map(|s| s.conditions.clone())
        .unwrap_or_default();
    update_conditions(
        &mut conditions,
        ready_condition(true, "Synced", "Language profile synchronized with Sonarr"),
    );

    let status = SonarrLanguageProfileStatus {
        conditions,
        id: sonarr_profile.id,
        observed_generation: profile.metadata.generation.unwrap_or(0),
    };

    let status_patch = serde_json::json!({ "status": status });
    api.patch_status(&name, &PatchParams::default(), &Patch::Merge(&status_patch))
        .await?;

    Ok(Action::requeue(REQUEUE_DURATION))
}

fn convert_language(lang: &LanguageType) -> Language {
    let mut language = Language::new();
    language.id = Some(get_language_id(lang));
    language.name = Some(Some(format!("{:?}", lang)));
    language
}

fn get_language_id(lang: &LanguageType) -> i32 {
    match lang {
        LanguageType::Unknown => -1,
        LanguageType::English => 1,
        LanguageType::French => 2,
        LanguageType::Spanish => 3,
        LanguageType::German => 4,
        LanguageType::Italian => 5,
        LanguageType::Danish => 6,
        LanguageType::Dutch => 7,
        LanguageType::Japanese => 8,
        LanguageType::Icelandic => 9,
        LanguageType::Chinese => 10,
        LanguageType::Russian => 11,
        LanguageType::Polish => 12,
        LanguageType::Vietnamese => 13,
        LanguageType::Swedish => 14,
        LanguageType::Norwegian => 15,
        LanguageType::Finnish => 16,
        LanguageType::Turkish => 17,
        LanguageType::Portuguese => 18,
        LanguageType::Flemish => 19,
        LanguageType::Greek => 20,
        LanguageType::Korean => 21,
        LanguageType::Hungarian => 22,
        LanguageType::Hebrew => 23,
        LanguageType::Lithuanian => 24,
        LanguageType::Czech => 25,
        LanguageType::Hindi => 26,
        LanguageType::Romanian => 27,
        LanguageType::Thai => 28,
        LanguageType::Bulgarian => 29,
        LanguageType::PortugueseBrazil => 30,
        LanguageType::Arabic => 31,
        LanguageType::Ukrainian => 32,
        LanguageType::Persian => 33,
        LanguageType::Bengali => 34,
        LanguageType::Slovak => 35,
        LanguageType::Latvian => 36,
        LanguageType::SpanishLatino => 37,
        LanguageType::Catalan => 38,
        LanguageType::Croatian => 39,
        LanguageType::Serbian => 40,
        LanguageType::Bosnian => 41,
        LanguageType::Estonian => 42,
        LanguageType::Tamil => 43,
        LanguageType::Indonesian => 44,
        LanguageType::Telugu => 45,
        LanguageType::Macedonian => 46,
        LanguageType::Slovenian => 47,
        LanguageType::Malay => 48,
        LanguageType::Original => -2,
        LanguageType::Any => 0,
    }
}

async fn reconcile_cleanup(
    profile: Arc<SonarrLanguageProfile>,
    ctx: Arc<Context>,
) -> Result<Action> {
    let client = &ctx.client;
    let namespace = profile
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;

    info!(
        "Cleaning up SonarrLanguageProfile: {}/{}",
        namespace,
        profile.name_any()
    );

    if let Some(id) = profile.status.as_ref().and_then(|s| s.id)
        && let Ok(config) =
            get_sonarr_config(&ctx, client, &namespace, &profile.spec.sonarr_instance_ref).await
    {
        let _ = language_profile_api::delete_language_profile(&config, id).await;
    }

    Ok(Action::await_change())
}
