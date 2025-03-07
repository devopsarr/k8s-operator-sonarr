use kube::CustomResourceExt;
use k8s_opeartor_sonarr::crd::sonarr::Sonarr;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate CRD definition
    let crd = Sonarr::crd();
    
    // Convert to YAML
    let yaml = serde_yaml::to_string(&crd)?;
    
    // Create manifests directory if it doesn't exist
    let path = Path::new("manifests");
    if !path.exists() {
        fs::create_dir(path)?;
    }
    
    // Write to file
    fs::write("manifests/crd_2.yaml", yaml)?;
    
    println!("Successfully generated CRD manifest at manifests/crd.yaml");
    
    Ok(())
}