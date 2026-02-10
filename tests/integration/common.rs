//! Common utilities for integration tests

use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use kube::{
    Client, Resource,
    api::{Api, DeleteParams, ListParams, Patch, PatchParams},
};
use std::time::Duration;
use tokio::time::timeout;

/// Default timeout for test operations
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Test namespace for integration tests
pub const TEST_NAMESPACE: &str = "sonarr-operator-test";

/// Create a test client from the current kubeconfig context
pub async fn test_client() -> Client {
    Client::try_default()
        .await
        .expect("Failed to create Kubernetes client - is your kubeconfig configured?")
}

/// Ensure the test namespace exists
pub async fn ensure_test_namespace(client: &Client) -> Result<(), kube::Error> {
    use k8s_openapi::api::core::v1::Namespace;
    use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    let namespaces: Api<Namespace> = Api::all(client.clone());

    let ns = Namespace {
        metadata: ObjectMeta {
            name: Some(TEST_NAMESPACE.to_string()),
            labels: Some(
                [(
                    "app.kubernetes.io/managed-by".to_string(),
                    "sonarr-operator-test".to_string(),
                )]
                .into(),
            ),
            ..Default::default()
        },
        ..Default::default()
    };

    let patch_params = PatchParams::apply("sonarr-operator-test").force();
    namespaces
        .patch(TEST_NAMESPACE, &patch_params, &Patch::Apply(&ns))
        .await?;

    Ok(())
}

/// Delete the test namespace and all its resources
pub async fn cleanup_test_namespace(client: &Client) -> Result<(), kube::Error> {
    use k8s_openapi::api::core::v1::Namespace;

    let namespaces: Api<Namespace> = Api::all(client.clone());

    match namespaces
        .delete(TEST_NAMESPACE, &DeleteParams::default())
        .await
    {
        Ok(_) => {
            // Wait for namespace to be deleted
            let lp = ListParams::default().fields(&format!("metadata.name={}", TEST_NAMESPACE));
            let _ = timeout(Duration::from_secs(60), async {
                loop {
                    match namespaces.list(&lp).await {
                        Ok(list) if list.items.is_empty() => break,
                        _ => tokio::time::sleep(Duration::from_secs(1)).await,
                    }
                }
            })
            .await;
        }
        Err(kube::Error::Api(err)) if err.code == 404 => {
            // Namespace doesn't exist, that's fine
        }
        Err(e) => return Err(e),
    }

    Ok(())
}

/// Check if a CRD is established (ready to use)
pub async fn is_crd_established(client: &Client, crd_name: &str) -> bool {
    let crds: Api<CustomResourceDefinition> = Api::all(client.clone());

    match crds.get(crd_name).await {
        Ok(crd) => {
            if let Some(status) = crd.status {
                if let Some(conditions) = status.conditions {
                    return conditions
                        .iter()
                        .any(|c| c.type_ == "Established" && c.status == "True");
                }
            }
            false
        }
        Err(_) => false,
    }
}

/// Wait for a CRD to be established
pub async fn wait_for_crd(client: &Client, crd_name: &str) -> Result<(), String> {
    timeout(DEFAULT_TIMEOUT, async {
        loop {
            if is_crd_established(client, crd_name).await {
                return;
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    })
    .await
    .map_err(|_| format!("Timeout waiting for CRD {} to be established", crd_name))
}

/// Apply a resource to the cluster
pub async fn apply_resource<T>(
    client: &Client,
    namespace: &str,
    resource: &T,
) -> Result<T, kube::Error>
where
    T: Resource<Scope = k8s_openapi::NamespaceResourceScope>
        + Clone
        + serde::Serialize
        + serde::de::DeserializeOwned
        + std::fmt::Debug,
    <T as Resource>::DynamicType: Default,
{
    let api: Api<T> = Api::namespaced(client.clone(), namespace);
    let name = resource.meta().name.clone().unwrap_or_default();

    let patch_params = PatchParams::apply("sonarr-operator-test").force();
    api.patch(&name, &patch_params, &Patch::Apply(resource))
        .await
}

/// Delete a resource from the cluster
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
        Err(kube::Error::Api(err)) if err.code == 404 => Ok(()), // Already deleted
        Err(e) => Err(e),
    }
}

/// Wait for a resource to have a specific condition
pub async fn wait_for_condition<T, F>(
    client: &Client,
    namespace: &str,
    name: &str,
    condition_fn: F,
) -> Result<T, String>
where
    T: Resource<Scope = k8s_openapi::NamespaceResourceScope>
        + Clone
        + serde::Serialize
        + serde::de::DeserializeOwned
        + std::fmt::Debug,
    <T as Resource>::DynamicType: Default,
    F: Fn(&T) -> bool,
{
    let api: Api<T> = Api::namespaced(client.clone(), namespace);

    timeout(DEFAULT_TIMEOUT, async {
        loop {
            match api.get(name).await {
                Ok(resource) => {
                    if condition_fn(&resource) {
                        return Ok(resource);
                    }
                }
                Err(_) => {}
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    })
    .await
    .map_err(|_| format!("Timeout waiting for condition on {}", name))?
}

/// Generate a unique test name to avoid conflicts between test runs
pub fn unique_name(prefix: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!("{}-{}", prefix, timestamp % 100000)
}
