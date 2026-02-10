//! CRD Generator Binary
//!
//! This binary generates the Kubernetes Custom Resource Definitions (CRDs) for the Sonarr operator.
//!
//! Usage:
//!   cargo run --bin crdgen                     # Output all CRDs to stdout (combined)
//!   cargo run --bin crdgen -- --split <dir>    # Write individual CRD files to directory
//!   cargo run --bin crdgen -- --single <name>  # Output a single CRD to stdout
//!
//! Examples:
//!   cargo run --bin crdgen > crds.yaml
//!   cargo run --bin crdgen -- --split deploy/crds
//!   cargo run --bin crdgen -- --single SonarrTag > tag-crd.yaml
//!
//! This outputs the CRDs as YAML which can be applied to a Kubernetes cluster:
//!   kubectl apply -f deploy/crds/

use kube::CustomResourceExt;
use std::fs;
use std::path::Path;

use sonarr_operator::crds;

/// CRD definition with metadata for file generation
struct CrdInfo {
    name: &'static str,
    filename: &'static str,
    crd: k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition,
}

/// Macro to generate CRD entries from type names.
/// This ensures we can't forget to add new CRDs - just add the type name once.
macro_rules! register_crds {
    ($($crd_type:ident => $filename:literal),* $(,)?) => {
        vec![
            $(
                CrdInfo {
                    name: stringify!($crd_type),
                    filename: $filename,
                    crd: crds::$crd_type::crd(),
                },
            )*
        ]
    };
}

fn get_all_crds() -> Vec<CrdInfo> {
    // To add a new CRD, simply add a new line here: TypeName => "filename.yaml"
    register_crds!(
        Sonarr => "sonarr.yaml",
        SonarrAutoTag => "autotag.yaml",
        SonarrCustomFormat => "customformat.yaml",
        SonarrDelayProfile => "delayprofile.yaml",
        SonarrDownloadClient => "downloadclient.yaml",
        SonarrDownloadClientConfig => "downloadclientconfig.yaml",
        SonarrImportList => "importlist.yaml",
        SonarrIndexer => "indexer.yaml",
        SonarrIndexerConfig => "indexerconfig.yaml",
        SonarrLanguageProfile => "languageprofile.yaml",
        SonarrMediaManagementConfig => "mediamanagementconfig.yaml",
        SonarrMetadata => "metadata.yaml",
        SonarrNamingConfig => "namingconfig.yaml",
        SonarrNotification => "notification.yaml",
        SonarrQualityDefinition => "qualitydefinition.yaml",
        SonarrQualityProfile => "qualityprofile.yaml",
        SonarrRootFolder => "rootfolder.yaml",
        SonarrSeries => "series.yaml",
        SonarrTag => "tag.yaml",
    )
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Check for --split flag (write individual files)
    if args.len() > 2 && args[1] == "--split" {
        let output_dir = &args[2];
        let path = Path::new(output_dir);

        // Create directory if it doesn't exist
        if let Err(e) = fs::create_dir_all(path) {
            eprintln!("Failed to create directory {}: {}", output_dir, e);
            std::process::exit(1);
        }

        let crds = get_all_crds();
        for crd_info in crds {
            let file_path = path.join(crd_info.filename);
            let yaml = serde_yaml::to_string(&crd_info.crd).unwrap();

            if let Err(e) = fs::write(&file_path, yaml) {
                eprintln!("Failed to write {}: {}", file_path.display(), e);
                std::process::exit(1);
            }
            eprintln!("Generated: {}", file_path.display());
        }

        eprintln!("All CRDs generated in {}/", output_dir);
        return;
    }

    // Check if user wants a single CRD
    if args.len() > 2 && args[1] == "--single" {
        let crd_name = &args[2];
        let crds = get_all_crds();

        if let Some(crd_info) = crds.iter().find(|c| c.name == crd_name) {
            print!("{}", serde_yaml::to_string(&crd_info.crd).unwrap());
            return;
        }

        eprintln!("Unknown CRD: {}", crd_name);
        let available: Vec<_> = crds.iter().map(|c| c.name).collect();
        eprintln!("Available CRDs: {}", available.join(", "));
        std::process::exit(1);
    }

    // Generate all CRDs to stdout (combined format)
    let crds = get_all_crds();
    for (i, crd_info) in crds.iter().enumerate() {
        if i > 0 {
            println!("---");
        }
        print!("{}", serde_yaml::to_string(&crd_info.crd).unwrap());
    }
}
