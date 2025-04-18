use anyhow::Result;
use clap::Parser;
use futures::StreamExt;
use kube::{
    runtime::{controller::Controller, watcher::Config},
    Client,
};
use std::sync::Arc;
use tokio::signal;
use tracing::{error, info, Level};
use tracing_subscriber::{fmt, EnvFilter};

mod controller;
mod crd;
mod error;

use controller::reconciler::{error_policy, reconcile, SonarrState};
use crd::sonarr::Sonarr;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Opts {
    #[arg(long)]
    namespace: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .with_writer(std::io::stdout) // Forza l'output su stdout
        .init();
    info!("Starting rust-k8s-operator");

    // Parse command-line arguments
    let opts = Opts::parse();

    // Create Kubernetes client
    let client = Client::try_default().await?;
    info!("Successfully connected to Kubernetes cluster");
    
    // Shared state for the controller
    let state = Arc::new(SonarrState::new(client.clone()));

    // Set up controller for our custom resource based on namespace option
    let sonarr_api = if let Some(namespace) = opts.namespace {
        info!("Watching Sonarr resources in namespace: {}", namespace);
        kube::Api::<Sonarr>::namespaced(client.clone(), &namespace)
    } else {
        info!("Watching Sonarr resources across all namespaces");
        kube::Api::<Sonarr>::all(client.clone())
    };

    // Run the controller
    Controller::new(sonarr_api, Config::default())
        .run(reconcile, error_policy, state)
        .for_each(|res| async move {
            match res {
                Ok(o) => {
                    info!("Reconciled: {:?}", o);
                }
                Err(e) => {
                    error!("Reconcile error: {:?}", e);
                }
            }
        })
        .await;

    // Wait for SIGTERM
    signal::ctrl_c().await?;
    info!("Shutting down rust-k8s-operator");

    Ok(())
}
