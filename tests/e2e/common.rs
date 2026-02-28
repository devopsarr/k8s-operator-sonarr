//! Common utilities for E2E tests

use k8s_openapi::api::core::v1::{Namespace, Secret};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::{
    Client, Resource,
    api::{Api, DeleteParams, Patch, PatchParams},
};
use std::collections::BTreeMap;
use std::time::Duration;
use tokio::time::timeout;

/// E2E test namespace
pub const E2E_NAMESPACE: &str = "sonarr-e2e-test";

/// Default timeout for E2E operations (longer than integration tests)
pub const E2E_TIMEOUT: Duration = Duration::from_secs(120);

/// Short timeout for quick checks
pub const QUICK_TIMEOUT: Duration = Duration::from_secs(30);

/// Sonarr service details for E2E tests
pub const SONARR_SERVICE_NAME: &str = "sonarr";
pub const SONARR_SERVICE_PORT: u16 = 8989;

/// Create a Kubernetes client
pub async fn e2e_client() -> Client {
    Client::try_default()
        .await
        .expect("Failed to create Kubernetes client - is your kubeconfig configured?")
}

/// Setup the E2E test namespace with Sonarr connection secret
pub async fn setup_e2e_namespace(client: &Client) -> Result<(), anyhow::Error> {
    // Create namespace
    let namespaces: Api<Namespace> = Api::all(client.clone());
    let ns = Namespace {
        metadata: ObjectMeta {
            name: Some(E2E_NAMESPACE.to_string()),
            labels: Some(BTreeMap::from([
                (
                    "app.kubernetes.io/managed-by".to_string(),
                    "sonarr-e2e".to_string(),
                ),
                ("test-type".to_string(), "e2e".to_string()),
            ])),
            ..Default::default()
        },
        ..Default::default()
    };

    let patch_params = PatchParams::apply("sonarr-e2e-test").force();
    namespaces
        .patch(E2E_NAMESPACE, &patch_params, &Patch::Apply(&ns))
        .await?;

    // Wait for namespace to be active
    tokio::time::sleep(Duration::from_secs(1)).await;

    Ok(())
}

/// Create the Sonarr connection secret for tests
pub async fn create_sonarr_secret(
    client: &Client,
    api_key: &str,
    url: &str,
) -> Result<(), anyhow::Error> {
    let secrets: Api<Secret> = Api::namespaced(client.clone(), E2E_NAMESPACE);

    let secret = Secret {
        metadata: ObjectMeta {
            name: Some("sonarr-api-key".to_string()),
            namespace: Some(E2E_NAMESPACE.to_string()),
            ..Default::default()
        },
        string_data: Some(BTreeMap::from([
            ("api-key".to_string(), api_key.to_string()),
            ("url".to_string(), url.to_string()),
        ])),
        ..Default::default()
    };

    let patch_params = PatchParams::apply("sonarr-e2e-test").force();
    secrets
        .patch("sonarr-api-key", &patch_params, &Patch::Apply(&secret))
        .await?;

    Ok(())
}

/// Cleanup the E2E test namespace
pub async fn cleanup_e2e_namespace(client: &Client) -> Result<(), anyhow::Error> {
    let namespaces: Api<Namespace> = Api::all(client.clone());

    match namespaces
        .delete(E2E_NAMESPACE, &DeleteParams::default())
        .await
    {
        Ok(_) => {
            // Wait for namespace deletion
            let _ = timeout(Duration::from_secs(60), async {
                loop {
                    match namespaces.get(E2E_NAMESPACE).await {
                        Err(_) => break,
                        Ok(_) => tokio::time::sleep(Duration::from_secs(1)).await,
                    }
                }
            })
            .await;
            Ok(())
        }
        Err(kube::Error::Api(err)) if err.code == 404 => Ok(()),
        Err(e) => Err(e.into()),
    }
}

/// Apply a namespaced resource
pub async fn apply_resource<T>(client: &Client, resource: &T) -> Result<T, kube::Error>
where
    T: Resource<Scope = k8s_openapi::NamespaceResourceScope>
        + Clone
        + serde::Serialize
        + serde::de::DeserializeOwned
        + std::fmt::Debug,
    <T as Resource>::DynamicType: Default,
{
    let namespace = resource
        .meta()
        .namespace
        .as_deref()
        .unwrap_or(E2E_NAMESPACE);
    let api: Api<T> = Api::namespaced(client.clone(), namespace);
    let name = resource.meta().name.clone().unwrap_or_default();

    let patch_params = PatchParams::apply("sonarr-e2e-test").force();
    api.patch(&name, &patch_params, &Patch::Apply(resource))
        .await
}

/// Delete a namespaced resource
pub async fn delete_resource<T>(
    client: &Client,
    namespace: &str,
    name: &str,
) -> Result<(), kube::Error>
where
    T: Resource<Scope = k8s_openapi::NamespaceResourceScope>
        + Clone
        + serde::Serialize
        + serde::de::DeserializeOwned
        + std::fmt::Debug,
    <T as Resource>::DynamicType: Default,
{
    let api: Api<T> = Api::namespaced(client.clone(), namespace);

    match api.delete(name, &DeleteParams::default()).await {
        Ok(_) => Ok(()),
        Err(kube::Error::Api(err)) if err.code == 404 => Ok(()),
        Err(e) => Err(e),
    }
}

/// Wait for a resource to have a Ready condition
pub async fn wait_for_ready<T>(
    client: &Client,
    namespace: &str,
    name: &str,
    timeout_duration: Duration,
) -> Result<T, anyhow::Error>
where
    T: Resource<Scope = k8s_openapi::NamespaceResourceScope>
        + Clone
        + serde::Serialize
        + serde::de::DeserializeOwned
        + std::fmt::Debug
        + HasConditions,
    <T as Resource>::DynamicType: Default,
{
    let api: Api<T> = Api::namespaced(client.clone(), namespace);

    timeout(timeout_duration, async {
        loop {
            match api.get(name).await {
                Ok(resource) => {
                    if resource.is_ready() {
                        return Ok(resource);
                    }
                    // Check for error conditions
                    if let Some(msg) = resource.get_error_message() {
                        return Err(anyhow::anyhow!("Resource has error condition: {}", msg));
                    }
                }
                Err(e) => {
                    tracing::debug!("Waiting for resource {}: {:?}", name, e);
                }
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    })
    .await
    .map_err(|_| anyhow::anyhow!("Timeout waiting for {} to be ready", name))?
}

/// Wait for a resource to be deleted
pub async fn wait_for_deletion<T>(
    client: &Client,
    namespace: &str,
    name: &str,
    timeout_duration: Duration,
) -> Result<(), anyhow::Error>
where
    T: Resource<Scope = k8s_openapi::NamespaceResourceScope>
        + Clone
        + serde::Serialize
        + serde::de::DeserializeOwned
        + std::fmt::Debug,
    <T as Resource>::DynamicType: Default,
{
    let api: Api<T> = Api::namespaced(client.clone(), namespace);

    timeout(timeout_duration, async {
        loop {
            match api.get(name).await {
                Err(kube::Error::Api(err)) if err.code == 404 => {
                    return Ok(());
                }
                _ => {
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            }
        }
    })
    .await
    .map_err(|_| anyhow::anyhow!("Timeout waiting for {} to be deleted", name))?
}

/// Trait for resources that have conditions in their status
pub trait HasConditions {
    fn is_ready(&self) -> bool;
    fn get_error_message(&self) -> Option<String>;
}

/// Generate a unique test name
pub fn unique_name(prefix: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!("{}-{}", prefix, timestamp % 100000)
}

/// Test context that tracks resources for cleanup
pub struct TestContext {
    pub client: Client,
    pub sonarr: super::SonarrTestClient,
    cleanup_resources: Vec<CleanupResource>,
}

struct CleanupResource {
    kind: String,
    namespace: String,
    name: String,
}

impl TestContext {
    pub async fn new() -> Result<Self, anyhow::Error> {
        let client = e2e_client().await;

        // Get Sonarr URL and API key from environment or port-forward
        let sonarr_url = std::env::var("SONARR_URL")
            .unwrap_or_else(|_| format!("http://{}:{}", SONARR_SERVICE_NAME, SONARR_SERVICE_PORT));
        let sonarr_api_key = std::env::var("SONARR_API_KEY")
            .expect("SONARR_API_KEY environment variable must be set for E2E tests");

        let sonarr = super::SonarrTestClient::new(&sonarr_url, &sonarr_api_key)?;

        Ok(Self {
            client,
            sonarr,
            cleanup_resources: Vec::new(),
        })
    }

    /// Register a resource for cleanup at the end of the test
    pub fn register_cleanup(&mut self, kind: &str, namespace: &str, name: &str) {
        self.cleanup_resources.push(CleanupResource {
            kind: kind.to_string(),
            namespace: namespace.to_string(),
            name: name.to_string(),
        });
    }

    /// Cleanup all registered resources (called automatically on drop or explicitly)
    pub async fn cleanup(&mut self) {
        use sonarr_operator::crds::*;

        // Cleanup in reverse order (dependencies last)
        for resource in self.cleanup_resources.drain(..).rev() {
            tracing::info!(
                "Cleaning up {} {}/{}",
                resource.kind,
                resource.namespace,
                resource.name
            );

            let result = match resource.kind.as_str() {
                "SonarrTag" => {
                    delete_resource::<SonarrTag>(&self.client, &resource.namespace, &resource.name)
                        .await
                }
                "SonarrRootFolder" => {
                    delete_resource::<SonarrRootFolder>(
                        &self.client,
                        &resource.namespace,
                        &resource.name,
                    )
                    .await
                }
                "SonarrQualityProfile" => {
                    delete_resource::<SonarrQualityProfile>(
                        &self.client,
                        &resource.namespace,
                        &resource.name,
                    )
                    .await
                }
                "SonarrQualityDefinition" => {
                    delete_resource::<SonarrQualityDefinition>(
                        &self.client,
                        &resource.namespace,
                        &resource.name,
                    )
                    .await
                }
                "SonarrAutoTag" => {
                    delete_resource::<SonarrAutoTag>(
                        &self.client,
                        &resource.namespace,
                        &resource.name,
                    )
                    .await
                }
                "SonarrCustomFormat" => {
                    delete_resource::<SonarrCustomFormat>(
                        &self.client,
                        &resource.namespace,
                        &resource.name,
                    )
                    .await
                }
                "SonarrDelayProfile" => {
                    delete_resource::<SonarrDelayProfile>(
                        &self.client,
                        &resource.namespace,
                        &resource.name,
                    )
                    .await
                }
                "SonarrNotification" => {
                    delete_resource::<SonarrNotification>(
                        &self.client,
                        &resource.namespace,
                        &resource.name,
                    )
                    .await
                }
                "SonarrIndexer" => {
                    delete_resource::<SonarrIndexer>(
                        &self.client,
                        &resource.namespace,
                        &resource.name,
                    )
                    .await
                }
                "SonarrDownloadClient" => {
                    delete_resource::<SonarrDownloadClient>(
                        &self.client,
                        &resource.namespace,
                        &resource.name,
                    )
                    .await
                }
                "SonarrMediaManagementConfig" => {
                    delete_resource::<SonarrMediaManagementConfig>(
                        &self.client,
                        &resource.namespace,
                        &resource.name,
                    )
                    .await
                }
                "SonarrNamingConfig" => {
                    delete_resource::<SonarrNamingConfig>(
                        &self.client,
                        &resource.namespace,
                        &resource.name,
                    )
                    .await
                }
                "SonarrIndexerConfig" => {
                    delete_resource::<SonarrIndexerConfig>(
                        &self.client,
                        &resource.namespace,
                        &resource.name,
                    )
                    .await
                }
                "SonarrDownloadClientConfig" => {
                    delete_resource::<SonarrDownloadClientConfig>(
                        &self.client,
                        &resource.namespace,
                        &resource.name,
                    )
                    .await
                }
                "SonarrImportList" => {
                    delete_resource::<SonarrImportList>(
                        &self.client,
                        &resource.namespace,
                        &resource.name,
                    )
                    .await
                }
                "SonarrSeries" => {
                    delete_resource::<SonarrSeries>(
                        &self.client,
                        &resource.namespace,
                        &resource.name,
                    )
                    .await
                }
                "SonarrMetadata" => {
                    delete_resource::<SonarrMetadata>(
                        &self.client,
                        &resource.namespace,
                        &resource.name,
                    )
                    .await
                }
                "SonarrLanguageProfile" => {
                    delete_resource::<SonarrLanguageProfile>(
                        &self.client,
                        &resource.namespace,
                        &resource.name,
                    )
                    .await
                }
                _ => {
                    tracing::warn!("Unknown resource kind for cleanup: {}", resource.kind);
                    Ok(())
                }
            };

            if let Err(e) = result {
                tracing::warn!(
                    "Failed to cleanup {} {}: {:?}",
                    resource.kind,
                    resource.name,
                    e
                );
            }
        }
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        // Note: async cleanup in drop is tricky, recommend calling cleanup() explicitly
        if !self.cleanup_resources.is_empty() {
            tracing::warn!(
                "TestContext dropped with {} resources not cleaned up. Call cleanup() explicitly.",
                self.cleanup_resources.len()
            );
        }
    }
}

// Implement HasConditions for our CRDs
macro_rules! impl_has_conditions {
    ($type:ty) => {
        impl HasConditions for $type {
            fn is_ready(&self) -> bool {
                self.status
                    .as_ref()
                    .map(|s| {
                        s.conditions
                            .iter()
                            .any(|c| c.type_ == "Ready" && c.status == "True")
                    })
                    .unwrap_or(false)
            }

            fn get_error_message(&self) -> Option<String> {
                self.status.as_ref().and_then(|s| {
                    s.conditions
                        .iter()
                        .find(|c| c.type_ == "Ready" && c.status == "False")
                        .map(|c| c.message.clone())
                })
            }
        }
    };
}

use sonarr_operator::crds::*;

impl_has_conditions!(SonarrTag);
impl_has_conditions!(SonarrRootFolder);
impl_has_conditions!(SonarrQualityProfile);
impl_has_conditions!(SonarrAutoTag);
impl_has_conditions!(SonarrCustomFormat);
impl_has_conditions!(SonarrDelayProfile);
impl_has_conditions!(SonarrNotification);
impl_has_conditions!(SonarrIndexer);
impl_has_conditions!(SonarrDownloadClient);
impl_has_conditions!(SonarrMediaManagementConfig);
impl_has_conditions!(SonarrNamingConfig);
impl_has_conditions!(SonarrIndexerConfig);
impl_has_conditions!(SonarrDownloadClientConfig);
impl_has_conditions!(SonarrQualityDefinition);
impl_has_conditions!(SonarrImportList);
impl_has_conditions!(SonarrSeries);
impl_has_conditions!(SonarrMetadata);
impl_has_conditions!(SonarrLanguageProfile);
