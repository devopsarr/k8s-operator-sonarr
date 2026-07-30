use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::SecretKeySelector;

/// Sonarr is the main CRD that deploys and manages a Sonarr instance
///
/// This CRD creates:
/// - A Deployment with the Sonarr container
/// - An init container for database migrations
/// - A Service to expose Sonarr
/// - A PersistentVolumeClaim for configuration storage
/// - Optional Ingress for external access
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "devopsarr.io",
    version = "v1alpha1",
    kind = "Sonarr",
    plural = "sonarrs",
    shortname = "snr",
    namespaced,
    status = "SonarrStatus",
    printcolumn = r#"{"name":"Ready","type":"string","jsonPath":".status.conditions[?(@.type==\"Ready\")].status"}"#,
    printcolumn = r#"{"name":"URL","type":"string","jsonPath":".status.url"}"#,
    printcolumn = r#"{"name":"Age","type":"date","jsonPath":".metadata.creationTimestamp"}"#
)]
#[serde(rename_all = "camelCase")]
pub struct SonarrSpec {
    /// Sonarr image to use (default: lscr.io/linuxserver/sonarr:latest)
    #[serde(default = "default_image")]
    pub image: String,

    /// Image pull policy (default: IfNotPresent)
    #[serde(default = "default_image_pull_policy")]
    pub image_pull_policy: String,

    /// Number of replicas (should be 1 for Sonarr)
    #[serde(default = "default_replicas")]
    pub replicas: i32,

    /// Storage configuration
    #[serde(default)]
    pub storage: StorageConfig,

    /// Service configuration
    #[serde(default)]
    pub service: ServiceConfig,

    /// Ingress configuration (optional)
    #[serde(default)]
    pub ingress: Option<IngressConfig>,

    /// HTTPRoute configuration for Gateway API (optional)
    #[serde(default)]
    pub http_route: Option<HTTPRouteConfig>,

    /// Environment variables
    #[serde(default)]
    pub env: Vec<EnvVar>,

    /// Resource requirements
    #[serde(default)]
    pub resources: Option<ResourceRequirements>,

    /// Volume mounts for media directories
    #[serde(default)]
    pub volume_mounts: Vec<VolumeMount>,

    /// Additional volumes
    #[serde(default)]
    pub volumes: Vec<Volume>,

    /// Node selector
    #[serde(default)]
    pub node_selector: std::collections::BTreeMap<String, String>,

    /// Tolerations
    #[serde(default)]
    pub tolerations: Vec<Toleration>,

    /// Pod security context
    #[serde(default)]
    pub security_context: Option<PodSecurityContext>,

    /// Init container configuration (for custom init logic)
    #[serde(default)]
    pub init_container: Option<InitContainerConfig>,

    /// API key secret reference (optional - will be auto-generated if not provided)
    #[serde(default)]
    pub api_key_secret_ref: Option<SecretKeySelector>,

    /// Sonarr application configuration (config.xml settings)
    #[serde(default)]
    pub config: SonarrConfig,
}

/// Configuration for Sonarr's config.xml
/// These settings are applied by the init container on startup
#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SonarrConfig {
    /// Init container image used to configure config.xml (default: busybox:latest)
    #[serde(default)]
    pub init_container_image: Option<String>,

    /// URL base for reverse proxy setups (e.g., "/sonarr")
    #[serde(default)]
    pub url_base: Option<String>,

    /// Bind address (default: "*")
    #[serde(default)]
    pub bind_address: Option<String>,

    /// Log level: trace, debug, info, warn, error (default: info)
    #[serde(default)]
    pub log_level: Option<String>,

    /// Instance name displayed in the UI
    #[serde(default)]
    pub instance_name: Option<String>,

    /// Authentication method: None, Basic, Forms, External (default: None)
    #[serde(default)]
    pub authentication_method: Option<String>,

    /// Authentication required for API access (default: false)
    #[serde(default)]
    pub authentication_required: Option<bool>,

    /// Analytics enabled (default: true)
    #[serde(default)]
    pub analytics_enabled: Option<bool>,
}

impl Default for SonarrSpec {
    fn default() -> Self {
        Self {
            image: default_image(),
            image_pull_policy: default_image_pull_policy(),
            replicas: default_replicas(),
            storage: StorageConfig::default(),
            service: ServiceConfig::default(),
            ingress: None,
            http_route: None,
            env: Vec::new(),
            resources: None,
            volume_mounts: Vec::new(),
            volumes: Vec::new(),
            node_selector: std::collections::BTreeMap::new(),
            tolerations: Vec::new(),
            security_context: None,
            init_container: None,
            api_key_secret_ref: None,
            config: SonarrConfig::default(),
        }
    }
}

fn default_image() -> String {
    "lscr.io/linuxserver/sonarr:latest".to_string()
}

fn default_image_pull_policy() -> String {
    "IfNotPresent".to_string()
}

fn default_replicas() -> i32 {
    1
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StorageConfig {
    /// Storage class for the PVC
    #[serde(default)]
    pub storage_class: Option<String>,

    /// Size of the config PVC (default: 1Gi)
    #[serde(default = "default_storage_size")]
    pub size: String,

    /// Access modes (default: ReadWriteOnce)
    #[serde(default = "default_access_modes")]
    pub access_modes: Vec<String>,

    /// Existing PVC to use (optional)
    #[serde(default)]
    pub existing_claim: Option<String>,
}

fn default_storage_size() -> String {
    "1Gi".to_string()
}

fn default_access_modes() -> Vec<String> {
    vec!["ReadWriteOnce".to_string()]
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ServiceConfig {
    /// Service type (default: ClusterIP)
    #[serde(default = "default_service_type")]
    pub service_type: String,

    /// Service port (default: 8989)
    #[serde(default = "default_service_port")]
    pub port: i32,

    /// Container port - the port Sonarr listens on inside the container (default: 8989)
    #[serde(default = "default_container_port")]
    pub container_port: i32,

    /// Node port (only for NodePort type)
    #[serde(default)]
    pub node_port: Option<i32>,

    /// Service annotations
    #[serde(default)]
    pub annotations: std::collections::BTreeMap<String, String>,
}

fn default_service_type() -> String {
    "ClusterIP".to_string()
}

fn default_service_port() -> i32 {
    8989
}

fn default_container_port() -> i32 {
    8989
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IngressConfig {
    /// Enable ingress (default: false)
    #[serde(default)]
    pub enabled: bool,

    /// Ingress class name
    #[serde(default)]
    pub ingress_class_name: Option<String>,

    /// Hostname for the ingress
    pub host: String,

    /// Path for the ingress (default: /)
    #[serde(default = "default_ingress_path")]
    pub path: String,

    /// Path type (default: Prefix)
    #[serde(default = "default_path_type")]
    pub path_type: String,

    /// TLS configuration
    #[serde(default)]
    pub tls: Option<IngressTLS>,

    /// Ingress annotations
    #[serde(default)]
    pub annotations: std::collections::BTreeMap<String, String>,
}

fn default_ingress_path() -> String {
    "/".to_string()
}

fn default_path_type() -> String {
    "Prefix".to_string()
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IngressTLS {
    /// Secret name containing TLS certificate
    pub secret_name: String,

    /// Hosts covered by the TLS certificate
    #[serde(default)]
    pub hosts: Vec<String>,
}

/// HTTPRoute configuration for Gateway API
#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HTTPRouteConfig {
    /// Enable HTTPRoute creation (default: false)
    #[serde(default)]
    pub enabled: bool,

    /// Gateway reference - the Gateway to attach to
    pub gateway_ref: GatewayRef,

    /// Hostnames for the HTTPRoute
    #[serde(default)]
    pub hostnames: Vec<String>,

    /// Path match for the route (default: /)
    #[serde(default = "default_http_route_path")]
    pub path: String,

    /// Path match type: Exact, PathPrefix, or RegularExpression (default: PathPrefix)
    #[serde(default = "default_http_route_path_type")]
    pub path_type: String,

    /// Additional labels for the HTTPRoute
    #[serde(default)]
    pub labels: std::collections::BTreeMap<String, String>,

    /// Additional annotations for the HTTPRoute
    #[serde(default)]
    pub annotations: std::collections::BTreeMap<String, String>,
}

/// Gateway reference for HTTPRoute
#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GatewayRef {
    /// Name of the Gateway
    pub name: String,

    /// Namespace of the Gateway (optional, defaults to same namespace as HTTPRoute)
    #[serde(default)]
    pub namespace: Option<String>,

    /// Section name within the Gateway (optional)
    #[serde(default)]
    pub section_name: Option<String>,
}

fn default_http_route_path() -> String {
    "/".to_string()
}

fn default_http_route_path_type() -> String {
    "PathPrefix".to_string()
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EnvVar {
    /// Name of the environment variable
    pub name: String,

    /// Value of the environment variable
    #[serde(default)]
    pub value: Option<String>,

    /// Reference to a secret or configmap
    #[serde(default)]
    pub value_from: Option<EnvVarSource>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EnvVarSource {
    /// Secret key reference
    #[serde(default)]
    pub secret_key_ref: Option<SecretKeySelector>,

    /// ConfigMap key reference
    #[serde(default)]
    pub config_map_key_ref: Option<ConfigMapKeySelector>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapKeySelector {
    /// Name of the configmap
    pub name: String,

    /// Key in the configmap
    pub key: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRequirements {
    /// Resource limits
    #[serde(default)]
    pub limits: std::collections::BTreeMap<String, String>,

    /// Resource requests
    #[serde(default)]
    pub requests: std::collections::BTreeMap<String, String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct VolumeMount {
    /// Name of the volume
    pub name: String,

    /// Mount path inside the container
    pub mount_path: String,

    /// Sub path (optional)
    #[serde(default)]
    pub sub_path: Option<String>,

    /// Read only flag
    #[serde(default)]
    pub read_only: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    /// Name of the volume
    pub name: String,

    /// PVC claim
    #[serde(default)]
    pub persistent_volume_claim: Option<PersistentVolumeClaimVolumeSource>,

    /// HostPath volume
    #[serde(default)]
    pub host_path: Option<HostPathVolumeSource>,

    /// NFS volume
    #[serde(default)]
    pub nfs: Option<NFSVolumeSource>,

    /// ConfigMap volume
    #[serde(default)]
    pub config_map: Option<ConfigMapVolumeSource>,

    /// Empty dir volume
    #[serde(default)]
    pub empty_dir: Option<EmptyDirVolumeSource>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeClaimVolumeSource {
    pub claim_name: String,
    #[serde(default)]
    pub read_only: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HostPathVolumeSource {
    pub path: String,
    #[serde(rename = "type", default)]
    pub host_path_type: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NFSVolumeSource {
    pub server: String,
    pub path: String,
    #[serde(default)]
    pub read_only: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapVolumeSource {
    pub name: String,
    #[serde(default)]
    pub items: Vec<KeyToPath>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeyToPath {
    pub key: String,
    pub path: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmptyDirVolumeSource {
    #[serde(default)]
    pub medium: Option<String>,
    #[serde(default)]
    pub size_limit: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Toleration {
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub operator: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub effect: Option<String>,
    #[serde(default)]
    pub toleration_seconds: Option<i64>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PodSecurityContext {
    #[serde(default)]
    pub run_as_user: Option<i64>,
    #[serde(default)]
    pub run_as_group: Option<i64>,
    #[serde(default)]
    pub fs_group: Option<i64>,
    #[serde(default)]
    pub run_as_non_root: Option<bool>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct InitContainerConfig {
    /// Image for init container (default: busybox:latest)
    #[serde(default = "default_init_image")]
    pub image: String,

    /// Command to run in init container
    #[serde(default)]
    pub command: Vec<String>,

    /// Arguments for the command
    #[serde(default)]
    pub args: Vec<String>,

    /// Environment variables for init container
    #[serde(default)]
    pub env: Vec<EnvVar>,
}

fn default_init_image() -> String {
    "busybox:latest".to_string()
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SonarrStatus {
    /// Current conditions
    #[serde(default)]
    pub conditions: Vec<Condition>,

    /// URL to access Sonarr
    #[serde(default)]
    pub url: Option<String>,

    /// API key (stored in secret)
    #[serde(default)]
    pub api_key_secret: Option<String>,

    /// Observed generation
    #[serde(default)]
    pub observed_generation: i64,

    /// Number of ready replicas
    #[serde(default)]
    pub ready_replicas: i32,

    /// Sonarr version
    #[serde(default)]
    pub version: Option<String>,
}

impl Sonarr {
    /// Get the service port (default: 8989)
    pub fn service_port(&self) -> i32 {
        if self.spec.service.port == 0 {
            8989
        } else {
            self.spec.service.port
        }
    }

    /// Get the container port (default: 8989)
    pub fn container_port(&self) -> i32 {
        if self.spec.service.container_port == 0 {
            8989
        } else {
            self.spec.service.container_port
        }
    }

    /// Get the service type (default: ClusterIP)
    pub fn service_type(&self) -> String {
        if self.spec.service.service_type.is_empty() {
            "ClusterIP".to_string()
        } else {
            self.spec.service.service_type.clone()
        }
    }

    /// Get the service name for this instance
    pub fn service_name(&self) -> String {
        format!(
            "{}-sonarr",
            self.metadata.name.as_deref().unwrap_or("unknown")
        )
    }

    /// Get the deployment name for this instance
    pub fn deployment_name(&self) -> String {
        format!(
            "{}-sonarr",
            self.metadata.name.as_deref().unwrap_or("unknown")
        )
    }

    /// Get the PVC name for this instance
    pub fn pvc_name(&self) -> String {
        format!(
            "{}-sonarr-config",
            self.metadata.name.as_deref().unwrap_or("unknown")
        )
    }

    /// Get the secret name for API key
    pub fn api_key_secret_name(&self) -> String {
        format!(
            "{}-sonarr-apikey",
            self.metadata.name.as_deref().unwrap_or("unknown")
        )
    }

    /// Get the internal URL for Sonarr
    pub fn internal_url(&self, namespace: &str) -> String {
        format!(
            "http://{}.{}.svc.cluster.local:{}",
            self.service_name(),
            namespace,
            self.service_port()
        )
    }
}
