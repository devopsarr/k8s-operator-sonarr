use kube::CustomResourceExt;
use k8s_opeartor_sonarr::crd::sonarr::Sonarr;
use std::fs;
use std::path::Path;

use tracing::{debug, Level};
use tracing_subscriber::{fmt, EnvFilter};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .with_writer(std::io::stdout) // Forza l'output su stdout
        .init();
    debug!("Build CRD...");
    // Generate CRD definition
    let crd = Sonarr::crd();
    
    // Convert to YAML
    let yaml = serde_yaml::to_string(&crd)?;
    
    // Create manifests directory if it doesn't exist
    let path = Path::new("manifests/crds");
    if !path.exists() {
        fs::create_dir(path)?;
    }
    
    // Write to file
    fs::write("manifests/crds/sonarr.yaml", yaml)?;
    
    debug!("Successfully generated CRD manifest at manifests/crds/sonarr.yaml");
    
    Ok(())
}