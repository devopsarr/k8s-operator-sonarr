use kube::Client;
use std::sync::Arc;
use tracing::{error, info};

use sonarr_operator::{Context, api, controllers};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Install the ring crypto provider for rustls
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sonarr_operator=info,kube=info".into()),
        )
        .json()
        .init();

    info!("Starting Sonarr Kubernetes Operator");

    // Create Kubernetes client
    let client = Client::try_default().await?;
    info!("Connected to Kubernetes cluster");

    // Create shared context
    let context = Arc::new(Context {
        client: client.clone(),
        sonarr_client_factory: Arc::new(api::SonarrClientFactory::new()),
    });

    // Start all controllers concurrently
    let sonarr_controller = controllers::sonarr::run(client.clone(), context.clone());
    let tag_controller = controllers::tag::run(client.clone(), context.clone());
    let root_folder_controller = controllers::root_folder::run(client.clone(), context.clone());
    let quality_profile_controller =
        controllers::quality_profile::run(client.clone(), context.clone());
    let indexer_controller = controllers::indexer::run(client.clone(), context.clone());
    let download_client_controller =
        controllers::download_client::run(client.clone(), context.clone());
    let notification_controller = controllers::notification::run(client.clone(), context.clone());
    let series_controller = controllers::series::run(client.clone(), context.clone());
    let import_list_controller = controllers::import_list::run(client.clone(), context.clone());
    let language_profile_controller =
        controllers::language_profile::run(client.clone(), context.clone());
    let metadata_controller = controllers::metadata::run(client.clone(), context.clone());
    let custom_format_controller = controllers::custom_format::run(client.clone(), context.clone());
    let delay_profile_controller = controllers::delay_profile::run(client.clone(), context.clone());
    let quality_definition_controller =
        controllers::quality_definition::run(client.clone(), context.clone());
    let auto_tag_controller = controllers::auto_tag::run(client.clone(), context.clone());

    // Config controllers (singleton per Sonarr instance)
    let media_management_config_controller =
        controllers::media_management_config::run(client.clone(), context.clone());
    let naming_config_controller = controllers::naming_config::run(client.clone(), context.clone());
    let indexer_config_controller =
        controllers::indexer_config::run(client.clone(), context.clone());
    let download_client_config_controller =
        controllers::download_client_config::run(client.clone(), context.clone());

    info!("All controllers started");

    // Run all controllers
    tokio::select! {
        _ = sonarr_controller => error!("Sonarr controller exited"),
        _ = tag_controller => error!("Tag controller exited"),
        _ = root_folder_controller => error!("Root folder controller exited"),
        _ = quality_profile_controller => error!("Quality profile controller exited"),
        _ = indexer_controller => error!("Indexer controller exited"),
        _ = download_client_controller => error!("Download client controller exited"),
        _ = notification_controller => error!("Notification controller exited"),
        _ = series_controller => error!("Series controller exited"),
        _ = import_list_controller => error!("Import list controller exited"),
        _ = language_profile_controller => error!("Language profile controller exited"),
        _ = metadata_controller => error!("Metadata controller exited"),
        _ = custom_format_controller => error!("Custom format controller exited"),
        _ = delay_profile_controller => error!("Delay profile controller exited"),
        _ = quality_definition_controller => error!("Quality definition controller exited"),
        _ = auto_tag_controller => error!("Auto tag controller exited"),
        _ = media_management_config_controller => error!("Media management config controller exited"),
        _ = naming_config_controller => error!("Naming config controller exited"),
        _ = indexer_config_controller => error!("Indexer config controller exited"),
        _ = download_client_config_controller => error!("Download client config controller exited"),
    }

    Ok(())
}
