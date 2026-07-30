//! CRD Generator Binary
//!
//! Generates the Kubernetes Custom Resource Definitions for the Sonarr operator.
//!
//! Usage:
//!   cargo run --bin crdgen                            # All CRDs to stdout (combined)
//!   cargo run --bin crdgen -- --split <dir>           # Plain CRD files (one per CRD)
//!   cargo run --bin crdgen -- --split <dir> --helm    # Helm-templated CRDs (gated + keep)
//!   cargo run --bin crdgen -- --single <name>         # Single CRD to stdout
//!
//! Examples:
//!   cargo run --bin crdgen > crds.yaml
//!   cargo run --bin crdgen -- --split charts/sonarr-operator/templates/crds --helm
//!   cargo run --bin crdgen -- --single SonarrTag > tag-crd.yaml

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

/// Inject Helm templating into a serialised CRD YAML document so it can be
/// rendered by the chart. The wrapper:
/// - guards the whole document on `.Values.crds.install`
/// - conditionally emits `helm.sh/resource-policy: keep` based on `.Values.crds.keep`
/// - merges user-supplied `.Values.crds.annotations` and `.Values.crds.additionalLabels`
///
/// The annotation injection is purely string-level (no extra deps): CRDs always
/// serialise with a top-level `metadata:` block produced by `serde_yaml`, so we
/// anchor on that.
fn wrap_for_helm(yaml: &str) -> String {
    const ANCHOR: &str = "metadata:\n";

    // Helm-templated metadata stanza to be injected after `metadata:`. It:
    // - emits an `annotations:` block only when `.Values.crds.keep` or
    //   `.Values.crds.annotations` are present (using `or`)
    // - merges user-provided annotations and labels via `toYaml` so callers
    //   can customize per-chart extra metadata
    const METADATA_INJECTION: &str = r#"  {{- if or .Values.crds.keep .Values.crds.annotations }}
  annotations:
    {{- if .Values.crds.keep }}
    helm.sh/resource-policy: keep
    {{- end }}
    {{- with .Values.crds.annotations }}
{{ toYaml . | indent 4 }}
    {{- end }}
  {{- end }}
  {{- with .Values.crds.additionalLabels }}
  labels:
{{ toYaml . | indent 4 }}
  {{- end }}
"#;

    let injected = match yaml.find(ANCHOR) {
        Some(idx) => {
            let insert_at = idx + ANCHOR.len();
            let mut s = String::with_capacity(yaml.len() + METADATA_INJECTION.len());
            s.push_str(&yaml[..insert_at]);
            s.push_str(METADATA_INJECTION);
            s.push_str(&yaml[insert_at..]);
            s
        }
        None => {
            eprintln!(
                "warning: could not find `metadata:` anchor; emitting CRD without helm metadata injection"
            );
            yaml.to_string()
        }
    };

    let mut out = String::with_capacity(injected.len() + 96);
    out.push_str("{{- if .Values.crds.install }}\n");
    out.push_str(&injected);
    if !injected.ends_with('\n') {
        out.push('\n');
    }
    out.push_str("{{- end }}\n");
    out
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Check for --split flag (write individual files)
    if args.len() > 2 && args[1] == "--split" {
        let output_dir = &args[2];
        let helm_mode = args.iter().any(|a| a == "--helm");
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
            let contents = if helm_mode {
                wrap_for_helm(&yaml)
            } else {
                yaml
            };

            if let Err(e) = fs::write(&file_path, contents) {
                eprintln!("Failed to write {}: {}", file_path.display(), e);
                std::process::exit(1);
            }
            eprintln!("Generated: {}", file_path.display());
        }

        eprintln!(
            "All CRDs generated in {}/{}",
            output_dir,
            if helm_mode { " (helm mode)" } else { "" }
        );
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
