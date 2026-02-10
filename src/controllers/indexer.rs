use std::sync::Arc;

use k8s_openapi::api::core::v1::Secret;
use kube::api::{Api, Patch, PatchParams};
use kube::runtime::controller::Action;
use kube::{Client, ResourceExt};
use tracing::info;

use sonarr::apis::indexer_api;
use sonarr::models::IndexerResource;

use crate::crds::{SonarrIndexer, SonarrIndexerStatus};
use crate::crds::indexer::IndexerType;
use crate::error::{Error, Result};
use crate::Context;

use super::traits::{run_controller, reconcile_with_finalizer, REQUEUE_DURATION};
use super::tag::get_sonarr_config;
use super::{ready_condition, update_conditions};

/// Start the SonarrIndexer controller
pub async fn run(client: Client, context: Arc<Context>) {
    run_controller::<SonarrIndexer, _, _>(
        client,
        context,
        "SonarrIndexer",
        reconcile,
    ).await;
}

async fn reconcile(obj: Arc<SonarrIndexer>, ctx: Arc<Context>) -> Result<Action> {
    reconcile_with_finalizer(obj, ctx, reconcile_apply, reconcile_cleanup).await
}

async fn reconcile_apply(indexer: Arc<SonarrIndexer>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = indexer.namespace().ok_or(Error::MissingObjectKey(".metadata.namespace"))?;
    let name = indexer.name_any();

    info!("Reconciling SonarrIndexer: {}/{}", namespace, name);

    let config = get_sonarr_config(&ctx, client, &namespace, &indexer.spec.sonarr_instance_ref).await?;

    // Resolve API key from secret if needed
    // TODO: Use the api_key in indexer fields/config when setting up the indexer resource
    let _api_key = resolve_secret_value(
        client,
        &namespace,
        indexer.spec.config.api_key.clone(),
        indexer.spec.config.api_key_secret_ref.as_ref()
    ).await?;

    // Build indexer resource
    let mut idx_resource = IndexerResource::new();
    idx_resource.name = Some(Some(indexer.spec.name.clone()));
    idx_resource.implementation = Some(Some(get_implementation_name(&indexer.spec.indexer_type).to_string()));
    idx_resource.enable_rss = Some(indexer.spec.enable_rss);
    idx_resource.enable_automatic_search = Some(indexer.spec.enable_automatic_search);
    idx_resource.enable_interactive_search = Some(indexer.spec.enable_interactive_search);
    idx_resource.priority = Some(indexer.spec.priority);
    idx_resource.tags = Some(Some(indexer.spec.tags.clone()));

    // Fields would need more complex mapping for the sonarr crate

    let sonarr_indexer = if let Some(id) = indexer.status.as_ref().and_then(|s| s.id) {
        idx_resource.id = Some(id);
        match indexer_api::update_indexer(&config, id, Some(false), Some(idx_resource.clone())).await {
            Ok(i) => i,
            Err(_) => {
                idx_resource.id = None;
                indexer_api::create_indexer(&config, Some(false), Some(idx_resource)).await?
            }
        }
    } else {
        let existing = indexer_api::list_indexer(&config).await?;
        if let Some(existing_idx) = existing.iter().find(|i| i.name.as_ref().and_then(|n| n.as_ref()) == Some(&indexer.spec.name)) {
            existing_idx.clone()
        } else {
            indexer_api::create_indexer(&config, Some(false), Some(idx_resource)).await?
        }
    };

    // Update status
    let indexers_api: Api<SonarrIndexer> = Api::namespaced(client.clone(), &namespace);
    let mut conditions = indexer.status.as_ref().map(|s| s.conditions.clone()).unwrap_or_default();
    update_conditions(&mut conditions, ready_condition(true, "Synced", "Indexer synchronized with Sonarr"));

    let status = SonarrIndexerStatus {
        conditions,
        id: sonarr_indexer.id,
        observed_generation: indexer.metadata.generation.unwrap_or(0),
    };

    let status_patch = serde_json::json!({ "status": status });
    indexers_api.patch_status(&name, &PatchParams::default(), &Patch::Merge(&status_patch)).await?;

    Ok(Action::requeue(REQUEUE_DURATION))
}

fn get_implementation_name(indexer_type: &IndexerType) -> &'static str {
    match indexer_type {
        IndexerType::Newznab => "Newznab",
        IndexerType::Torznab => "Torznab",
        IndexerType::Fanzub => "Fanzub",
        IndexerType::BroadcastheNet => "BroadcastheNet",
        IndexerType::FileList => "FileList",
        IndexerType::HDBits => "HDBits",
        IndexerType::IPTorrents => "IPTorrents",
        IndexerType::Nyaa => "Nyaa",
        IndexerType::TorrentRss => "TorrentRssIndexer",
        IndexerType::TorrentLeech => "TorrentLeech",
    }
}

pub async fn resolve_secret_value(
    client: &Client,
    namespace: &str,
    direct_value: Option<String>,
    secret_ref: Option<&crate::crds::SecretKeySelector>,
) -> Result<Option<String>> {
    if let Some(value) = direct_value {
        return Ok(Some(value));
    }

    if let Some(ref secret_ref) = secret_ref {
        let secrets: Api<Secret> = Api::namespaced(client.clone(), namespace);
        let secret = secrets.get(&secret_ref.name).await
            .map_err(|_| Error::MissingApiCredentials)?;

        let data = secret.data.ok_or(Error::MissingApiCredentials)?;
        let value_bytes = data.get(&secret_ref.key).ok_or(Error::MissingApiCredentials)?;

        let value = String::from_utf8(value_bytes.0.clone())
            .map_err(|_| Error::MissingApiCredentials)?;

        return Ok(Some(value));
    }

    Ok(None)
}

async fn reconcile_cleanup(indexer: Arc<SonarrIndexer>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = indexer.namespace().ok_or(Error::MissingObjectKey(".metadata.namespace"))?;

    info!("Cleaning up SonarrIndexer: {}/{}", namespace, indexer.name_any());

    if let Some(id) = indexer.status.as_ref().and_then(|s| s.id) {
        if let Ok(config) = get_sonarr_config(&ctx, client, &namespace, &indexer.spec.sonarr_instance_ref).await {
            let _ = indexer_api::delete_indexer(&config, id).await;
        }
    }

    Ok(Action::await_change())
}
