use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod auto_tag;
pub mod custom_format;
pub mod delay_profile;
pub mod download_client;
pub mod download_client_config;
pub mod import_list;
pub mod indexer;
pub mod indexer_config;
pub mod language_profile;
pub mod media_management_config;
pub mod metadata;
pub mod naming_config;
pub mod notification;
pub mod quality_definition;
pub mod quality_profile;
pub mod root_folder;
pub mod series;
pub mod sonarr;
pub mod tag;

pub use auto_tag::{SonarrAutoTag, SonarrAutoTagSpec, SonarrAutoTagStatus};
pub use custom_format::{SonarrCustomFormat, SonarrCustomFormatSpec, SonarrCustomFormatStatus};
pub use delay_profile::{SonarrDelayProfile, SonarrDelayProfileSpec, SonarrDelayProfileStatus};
pub use download_client::{
    SonarrDownloadClient, SonarrDownloadClientSpec, SonarrDownloadClientStatus,
};
pub use import_list::{SonarrImportList, SonarrImportListSpec, SonarrImportListStatus};
pub use indexer::{SonarrIndexer, SonarrIndexerSpec, SonarrIndexerStatus};
pub use language_profile::{
    SonarrLanguageProfile, SonarrLanguageProfileSpec, SonarrLanguageProfileStatus,
};
pub use metadata::{SonarrMetadata, SonarrMetadataSpec, SonarrMetadataStatus};
pub use notification::{SonarrNotification, SonarrNotificationSpec, SonarrNotificationStatus};
pub use quality_definition::{
    SonarrQualityDefinition, SonarrQualityDefinitionSpec, SonarrQualityDefinitionStatus,
};
pub use quality_profile::{
    SonarrQualityProfile, SonarrQualityProfileSpec, SonarrQualityProfileStatus,
};
pub use root_folder::{SonarrRootFolder, SonarrRootFolderSpec, SonarrRootFolderStatus};
pub use series::{SonarrSeries, SonarrSeriesSpec, SonarrSeriesStatus};
pub use sonarr::{ServiceConfig, Sonarr, SonarrSpec, SonarrStatus, StorageConfig};
pub use tag::{SonarrTag, SonarrTagSpec, SonarrTagStatus};

// Config CRDs (singleton per Sonarr instance)
pub use download_client_config::{
    SonarrDownloadClientConfig, SonarrDownloadClientConfigSpec, SonarrDownloadClientConfigStatus,
};
pub use indexer_config::{SonarrIndexerConfig, SonarrIndexerConfigSpec, SonarrIndexerConfigStatus};
pub use media_management_config::{
    SonarrMediaManagementConfig, SonarrMediaManagementConfigSpec, SonarrMediaManagementConfigStatus,
};
pub use naming_config::{SonarrNamingConfig, SonarrNamingConfigSpec, SonarrNamingConfigStatus};

/// Common SecretKeySelector used across CRDs
#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SecretKeySelector {
    /// Name of the secret
    pub name: String,

    /// Key in the secret
    pub key: String,
}

/// Reference to a Sonarr instance used by sub-resources
#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SonarrInstanceRef {
    /// Name of the SonarrInstance resource
    #[serde(default)]
    pub name: String,

    /// Namespace of the SonarrInstance (optional, defaults to same namespace)
    #[serde(default)]
    pub namespace: Option<String>,
}

/// Base status fields shared by all Sonarr sub-resources (tags, notifications, etc.)
/// This provides a consistent status structure with conditions, id, and observed generation.
#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SonarrSubResourceStatus {
    /// Current conditions
    #[serde(default)]
    pub conditions: Vec<Condition>,

    /// Sonarr resource ID
    #[serde(default)]
    pub id: Option<i32>,

    /// Observed generation
    #[serde(default)]
    pub observed_generation: i64,
}

/// Common constants for the operator
pub const FINALIZER: &str = "sonarr.io/finalizer";
pub const SONARR_GROUP: &str = "devopsarr.io";
pub const SONARR_VERSION: &str = "v1alpha1";

/// Common labels
pub const LABEL_APP: &str = "app.kubernetes.io/name";
pub const LABEL_INSTANCE: &str = "app.kubernetes.io/instance";
pub const LABEL_MANAGED_BY: &str = "app.kubernetes.io/managed-by";
pub const LABEL_COMPONENT: &str = "app.kubernetes.io/component";

/// Annotations
pub const ANNOTATION_SONARR_REF: &str = "sonarr.io/instance";
pub const ANNOTATION_RESOURCE_ID: &str = "sonarr.io/resource-id";
