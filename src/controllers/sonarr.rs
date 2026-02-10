use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use futures::StreamExt;
use k8s_openapi::api::apps::v1::{Deployment, DeploymentSpec};
use k8s_openapi::api::core::v1::{
    Container, ContainerPort, EnvVar, PersistentVolumeClaim, PersistentVolumeClaimSpec, PodSpec,
    PodTemplateSpec, ResourceRequirements, Secret, Service, ServicePort, ServiceSpec, Volume,
    VolumeMount, VolumeResourceRequirements,
};
use k8s_openapi::api::networking::v1::{
    HTTPIngressPath, HTTPIngressRuleValue, Ingress, IngressBackend, IngressRule,
    IngressServiceBackend, IngressSpec, IngressTLS, ServiceBackendPort,
};
use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{LabelSelector, ObjectMeta, OwnerReference};
use kube::api::{Api, Patch, PatchParams, PostParams};
use kube::runtime::controller::{Action, Controller};
use kube::runtime::{
    finalizer::{Event, finalizer},
    watcher,
};
use kube::{Client, Resource, ResourceExt};
use tracing::{debug, error, info};

use crate::Context;
use crate::crds::{FINALIZER, LABEL_APP, LABEL_INSTANCE, LABEL_MANAGED_BY, Sonarr, SonarrStatus};
use crate::error::{Error, Result};

use super::{progressing_condition, ready_condition, update_conditions};

/// Start the Sonarr controller
pub async fn run(client: Client, context: Arc<Context>) {
    let sonarrs = Api::<Sonarr>::all(client.clone());

    info!("Starting Sonarr controller");

    Controller::new(sonarrs, watcher::Config::default())
        .shutdown_on_signal()
        .run(reconcile, error_policy, context)
        .for_each(|res| async move {
            match res {
                Ok(o) => debug!("Reconciled Sonarr: {:?}", o),
                Err(e) => error!("Reconcile error: {:?}", e),
            }
        })
        .await;
}

/// Error policy for the controller
fn error_policy(obj: Arc<Sonarr>, error: &Error, _ctx: Arc<Context>) -> Action {
    error!("Error reconciling Sonarr {}: {:?}", obj.name_any(), error);
    Action::requeue(Duration::from_secs(60))
}

/// Main reconciliation function
async fn reconcile(obj: Arc<Sonarr>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = obj
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;
    let name = obj.name_any();

    info!("Reconciling Sonarr: {}/{}", namespace, name);

    let instances: Api<Sonarr> = Api::namespaced(client.clone(), &namespace);

    // Handle finalizer
    finalizer(&instances, FINALIZER, obj.clone(), |event| async {
        match event {
            Event::Apply(instance) => reconcile_apply(instance, ctx.clone()).await,
            Event::Cleanup(instance) => reconcile_cleanup(instance, ctx.clone()).await,
        }
    })
    .await
    .map_err(|e| Error::FinalizerError(Box::new(e)))
}

/// Reconcile on apply (create/update)
async fn reconcile_apply(instance: Arc<Sonarr>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    let namespace = instance
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;
    let name = instance.name_any();

    // Create owner reference
    let owner_ref = instance.controller_owner_ref(&()).unwrap();

    // Create labels
    let labels = create_labels(&name);

    // Reconcile PVC
    reconcile_pvc(client, &namespace, &instance, &owner_ref, &labels).await?;

    // Reconcile Secret (API key)
    reconcile_secret(client, &namespace, &instance, &owner_ref, &labels).await?;

    // Reconcile Deployment
    reconcile_deployment(client, &namespace, &instance, &owner_ref, &labels).await?;

    // Reconcile Service
    reconcile_service(client, &namespace, &instance, &owner_ref, &labels).await?;

    // Reconcile Ingress (if configured)
    if let Some(ref ingress_config) = instance.spec.ingress {
        if ingress_config.enabled {
            reconcile_ingress(
                client,
                &namespace,
                &instance,
                ingress_config,
                &owner_ref,
                &labels,
            )
            .await?;
        }
    }

    // Reconcile HTTPRoute (if configured)
    if let Some(ref http_route_config) = instance.spec.http_route {
        if http_route_config.enabled {
            reconcile_http_route(
                client,
                &namespace,
                &instance,
                http_route_config,
                &owner_ref,
                &labels,
            )
            .await?;
        }
    }

    // Update status
    update_status(client, &namespace, &instance).await?;

    Ok(Action::requeue(Duration::from_secs(300)))
}

/// Reconcile on cleanup (delete)
async fn reconcile_cleanup(instance: Arc<Sonarr>, _ctx: Arc<Context>) -> Result<Action> {
    info!(
        "Cleaning up Sonarr: {}/{}",
        instance.namespace().unwrap_or_default(),
        instance.name_any()
    );

    // Resources are cleaned up automatically via owner references
    Ok(Action::await_change())
}

fn create_labels(name: &str) -> BTreeMap<String, String> {
    let mut labels = BTreeMap::new();
    labels.insert(LABEL_APP.to_string(), "sonarr".to_string());
    labels.insert(LABEL_INSTANCE.to_string(), name.to_string());
    labels.insert(LABEL_MANAGED_BY.to_string(), "sonarr-operator".to_string());
    labels
}

async fn reconcile_pvc(
    client: &Client,
    namespace: &str,
    instance: &Sonarr,
    owner_ref: &OwnerReference,
    labels: &BTreeMap<String, String>,
) -> Result<()> {
    // Skip if using existing claim
    if instance.spec.storage.existing_claim.is_some() {
        return Ok(());
    }

    let pvc_name = instance.pvc_name();
    let pvc_api: Api<PersistentVolumeClaim> = Api::namespaced(client.clone(), namespace);

    // Check if PVC exists
    if pvc_api.get_opt(&pvc_name).await?.is_some() {
        debug!("PVC {} already exists", pvc_name);
        return Ok(());
    }

    info!("Creating PVC: {}", pvc_name);

    // Use defaults if values are empty
    let storage_size = if instance.spec.storage.size.is_empty() {
        "1Gi".to_string()
    } else {
        instance.spec.storage.size.clone()
    };

    let access_modes = if instance.spec.storage.access_modes.is_empty() {
        vec!["ReadWriteOnce".to_string()]
    } else {
        instance.spec.storage.access_modes.clone()
    };

    let mut requests = BTreeMap::new();
    requests.insert("storage".to_string(), Quantity(storage_size));

    let pvc = PersistentVolumeClaim {
        metadata: ObjectMeta {
            name: Some(pvc_name.clone()),
            namespace: Some(namespace.to_string()),
            labels: Some(labels.clone()),
            owner_references: Some(vec![owner_ref.clone()]),
            ..Default::default()
        },
        spec: Some(PersistentVolumeClaimSpec {
            access_modes: Some(access_modes),
            storage_class_name: instance.spec.storage.storage_class.clone(),
            resources: Some(VolumeResourceRequirements {
                requests: Some(requests),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };

    pvc_api
        .create(&PostParams::default(), &pvc)
        .await
        .map_err(Error::KubeError)?;

    Ok(())
}

async fn reconcile_secret(
    client: &Client,
    namespace: &str,
    instance: &Sonarr,
    owner_ref: &OwnerReference,
    labels: &BTreeMap<String, String>,
) -> Result<()> {
    // Skip if using existing secret
    if instance.spec.api_key_secret_ref.is_some() {
        return Ok(());
    }

    let secret_name = instance.api_key_secret_name();
    let secret_api: Api<Secret> = Api::namespaced(client.clone(), namespace);

    // Check if secret exists
    if secret_api.get_opt(&secret_name).await?.is_some() {
        debug!("Secret {} already exists", secret_name);
        return Ok(());
    }

    info!("Creating API key secret: {}", secret_name);

    // Generate a random API key
    let api_key = generate_api_key();

    let mut data = BTreeMap::new();
    data.insert(
        "api-key".to_string(),
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &api_key),
    );

    let secret = Secret {
        metadata: ObjectMeta {
            name: Some(secret_name.clone()),
            namespace: Some(namespace.to_string()),
            labels: Some(labels.clone()),
            owner_references: Some(vec![owner_ref.clone()]),
            ..Default::default()
        },
        string_data: Some({
            let mut sd = BTreeMap::new();
            sd.insert("api-key".to_string(), api_key);
            sd
        }),
        ..Default::default()
    };

    secret_api
        .create(&PostParams::default(), &secret)
        .await
        .map_err(Error::KubeError)?;

    Ok(())
}

fn generate_api_key() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:032x}", timestamp)
}

async fn reconcile_deployment(
    client: &Client,
    namespace: &str,
    instance: &Sonarr,
    owner_ref: &OwnerReference,
    labels: &BTreeMap<String, String>,
) -> Result<()> {
    let deployment_name = instance.deployment_name();
    let deployment_api: Api<Deployment> = Api::namespaced(client.clone(), namespace);

    let deployment = build_deployment(instance, namespace, owner_ref, labels)?;

    let patch_params = PatchParams::apply("sonarr-operator").force();
    deployment_api
        .patch(&deployment_name, &patch_params, &Patch::Apply(&deployment))
        .await
        .map_err(Error::KubeError)?;

    info!("Deployment {} applied", deployment_name);

    Ok(())
}

fn build_deployment(
    instance: &Sonarr,
    namespace: &str,
    owner_ref: &OwnerReference,
    labels: &BTreeMap<String, String>,
) -> Result<Deployment> {
    let deployment_name = instance.deployment_name();
    let pvc_name = instance
        .spec
        .storage
        .existing_claim
        .clone()
        .unwrap_or_else(|| instance.pvc_name());

    // Build environment variables
    let mut env_vars: Vec<EnvVar> = vec![
        EnvVar {
            name: "PUID".to_string(),
            value: Some("1000".to_string()),
            ..Default::default()
        },
        EnvVar {
            name: "PGID".to_string(),
            value: Some("1000".to_string()),
            ..Default::default()
        },
        EnvVar {
            name: "TZ".to_string(),
            value: Some("Etc/UTC".to_string()),
            ..Default::default()
        },
    ];

    // Add user-defined environment variables
    for env in &instance.spec.env {
        let env_var = if let Some(ref value) = env.value {
            EnvVar {
                name: env.name.clone(),
                value: Some(value.clone()),
                ..Default::default()
            }
        } else if let Some(ref value_from) = env.value_from {
            let mut ev = EnvVar {
                name: env.name.clone(),
                ..Default::default()
            };
            if let Some(ref secret_ref) = value_from.secret_key_ref {
                ev.value_from = Some(k8s_openapi::api::core::v1::EnvVarSource {
                    secret_key_ref: Some(k8s_openapi::api::core::v1::SecretKeySelector {
                        name: secret_ref.name.clone(),
                        key: secret_ref.key.clone(),
                        optional: Some(false),
                    }),
                    ..Default::default()
                });
            } else if let Some(ref cm_ref) = value_from.config_map_key_ref {
                ev.value_from = Some(k8s_openapi::api::core::v1::EnvVarSource {
                    config_map_key_ref: Some(k8s_openapi::api::core::v1::ConfigMapKeySelector {
                        name: cm_ref.name.clone(),
                        key: cm_ref.key.clone(),
                        optional: Some(false),
                    }),
                    ..Default::default()
                });
            }
            ev
        } else {
            continue;
        };
        env_vars.push(env_var);
    }

    // Build volume mounts
    let mut volume_mounts: Vec<VolumeMount> = vec![VolumeMount {
        name: "config".to_string(),
        mount_path: "/config".to_string(),
        ..Default::default()
    }];

    for vm in &instance.spec.volume_mounts {
        volume_mounts.push(VolumeMount {
            name: vm.name.clone(),
            mount_path: vm.mount_path.clone(),
            sub_path: vm.sub_path.clone(),
            read_only: Some(vm.read_only),
            ..Default::default()
        });
    }

    // Build volumes
    let mut volumes: Vec<Volume> = vec![Volume {
        name: "config".to_string(),
        persistent_volume_claim: Some(
            k8s_openapi::api::core::v1::PersistentVolumeClaimVolumeSource {
                claim_name: pvc_name,
                read_only: Some(false),
            },
        ),
        ..Default::default()
    }];

    for v in &instance.spec.volumes {
        let mut volume = Volume {
            name: v.name.clone(),
            ..Default::default()
        };
        if let Some(ref pvc) = v.persistent_volume_claim {
            volume.persistent_volume_claim = Some(
                k8s_openapi::api::core::v1::PersistentVolumeClaimVolumeSource {
                    claim_name: pvc.claim_name.clone(),
                    read_only: Some(pvc.read_only),
                },
            );
        }
        if let Some(ref hp) = v.host_path {
            volume.host_path = Some(k8s_openapi::api::core::v1::HostPathVolumeSource {
                path: hp.path.clone(),
                type_: hp.host_path_type.clone(),
            });
        }
        if let Some(ref nfs) = v.nfs {
            volume.nfs = Some(k8s_openapi::api::core::v1::NFSVolumeSource {
                server: nfs.server.clone(),
                path: nfs.path.clone(),
                read_only: Some(nfs.read_only),
            });
        }
        if let Some(ref ed) = v.empty_dir {
            volume.empty_dir = Some(k8s_openapi::api::core::v1::EmptyDirVolumeSource {
                medium: ed.medium.clone(),
                size_limit: ed.size_limit.clone().map(Quantity),
            });
        }
        volumes.push(volume);
    }

    // Determine which secret to use for API key
    let api_key_secret_name = if let Some(ref secret_ref) = instance.spec.api_key_secret_ref {
        secret_ref.name.clone()
    } else {
        instance.api_key_secret_name()
    };
    let api_key_secret_key = instance
        .spec
        .api_key_secret_ref
        .as_ref()
        .map(|s| s.key.clone())
        .unwrap_or_else(|| "api-key".to_string());

    // Build init containers
    let mut init_containers = Vec::new();

    // Add config initialization init container using sed for XML manipulation
    // This container creates/updates config.xml with settings from the operator
    let config_script = r#"#!/bin/sh
set -e
CONFIG_FILE="/config/config.xml"
API_KEY="${SONARR_API_KEY}"

if [ -z "$API_KEY" ]; then
    echo "Error: SONARR_API_KEY environment variable is not set"
    exit 1
fi

# Function to set or update an XML element using sed
set_config_value() {
    element="$1"
    value="$2"

    if grep -q "<${element}>" "$CONFIG_FILE"; then
        # Element exists, update it
        sed -i "s|<${element}>.*</${element}>|<${element}>${value}</${element}>|g" "$CONFIG_FILE"
        echo "Updated ${element}"
    else
        # Element doesn't exist, add it before </Config>
        sed -i "s|</Config>|  <${element}>${value}</${element}>\n</Config>|" "$CONFIG_FILE"
        echo "Added ${element}"
    fi
}

if [ ! -f "$CONFIG_FILE" ]; then
    # Create minimal config.xml
    cat > "$CONFIG_FILE" << 'XMLEOF'
<?xml version="1.0" encoding="utf-8"?>
<Config>
  <LogLevel>info</LogLevel>
  <UrlBase></UrlBase>
  <BindAddress>*</BindAddress>
  <Port>8989</Port>
  <SslPort>9898</SslPort>
  <EnableSsl>False</EnableSsl>
  <LaunchBrowser>False</LaunchBrowser>
  <AuthenticationMethod>None</AuthenticationMethod>
  <UpdateMechanism>Docker</UpdateMechanism>
  <InstanceName>Sonarr</InstanceName>
</Config>
XMLEOF
    echo "Created new config.xml"
fi

# Set the API key
set_config_value "ApiKey" "$API_KEY"

# Set additional config values from environment if provided
[ -n "$SONARR_PORT" ] && set_config_value "Port" "$SONARR_PORT"
[ -n "$SONARR_URL_BASE" ] && set_config_value "UrlBase" "$SONARR_URL_BASE"
[ -n "$SONARR_BIND_ADDRESS" ] && set_config_value "BindAddress" "$SONARR_BIND_ADDRESS"
[ -n "$SONARR_LOG_LEVEL" ] && set_config_value "LogLevel" "$SONARR_LOG_LEVEL"
[ -n "$SONARR_INSTANCE_NAME" ] && set_config_value "InstanceName" "$SONARR_INSTANCE_NAME"
[ -n "$SONARR_AUTH_METHOD" ] && set_config_value "AuthenticationMethod" "$SONARR_AUTH_METHOD"
[ -n "$SONARR_AUTH_REQUIRED" ] && set_config_value "AuthenticationRequired" "$SONARR_AUTH_REQUIRED"
[ -n "$SONARR_ANALYTICS" ] && set_config_value "AnalyticsEnabled" "$SONARR_ANALYTICS"

# Ensure proper permissions
chmod 644 "$CONFIG_FILE"
echo "Config initialization complete"
"#;

    // Build environment variables for init container
    let mut init_env = vec![EnvVar {
        name: "SONARR_API_KEY".to_string(),
        value_from: Some(k8s_openapi::api::core::v1::EnvVarSource {
            secret_key_ref: Some(k8s_openapi::api::core::v1::SecretKeySelector {
                name: api_key_secret_name.clone(),
                key: api_key_secret_key.clone(),
                optional: Some(false),
            }),
            ..Default::default()
        }),
        ..Default::default()
    }];

    // Add port if non-default
    let container_port = instance.container_port();
    if container_port != 8989 {
        init_env.push(EnvVar {
            name: "SONARR_PORT".to_string(),
            value: Some(container_port.to_string()),
            ..Default::default()
        });
    }

    // Add config options from spec.config
    if let Some(ref url_base) = instance.spec.config.url_base {
        init_env.push(EnvVar {
            name: "SONARR_URL_BASE".to_string(),
            value: Some(url_base.clone()),
            ..Default::default()
        });
    }
    if let Some(ref bind_address) = instance.spec.config.bind_address {
        init_env.push(EnvVar {
            name: "SONARR_BIND_ADDRESS".to_string(),
            value: Some(bind_address.clone()),
            ..Default::default()
        });
    }
    if let Some(ref log_level) = instance.spec.config.log_level {
        init_env.push(EnvVar {
            name: "SONARR_LOG_LEVEL".to_string(),
            value: Some(log_level.clone()),
            ..Default::default()
        });
    }
    if let Some(ref instance_name) = instance.spec.config.instance_name {
        init_env.push(EnvVar {
            name: "SONARR_INSTANCE_NAME".to_string(),
            value: Some(instance_name.clone()),
            ..Default::default()
        });
    }
    if let Some(ref auth_method) = instance.spec.config.authentication_method {
        init_env.push(EnvVar {
            name: "SONARR_AUTH_METHOD".to_string(),
            value: Some(auth_method.clone()),
            ..Default::default()
        });
    }
    if let Some(auth_required) = instance.spec.config.authentication_required {
        init_env.push(EnvVar {
            name: "SONARR_AUTH_REQUIRED".to_string(),
            value: Some(if auth_required { "True" } else { "False" }.to_string()),
            ..Default::default()
        });
    }
    if let Some(analytics) = instance.spec.config.analytics_enabled {
        init_env.push(EnvVar {
            name: "SONARR_ANALYTICS".to_string(),
            value: Some(if analytics { "True" } else { "False" }.to_string()),
            ..Default::default()
        });
    }

    let init_image = instance
        .spec
        .config
        .init_container_image
        .clone()
        .unwrap_or_else(|| "busybox:latest".to_string());

    init_containers.push(Container {
        name: "init-config".to_string(),
        image: Some(init_image), // Busybox with sed (configurable)
        command: Some(vec![
            "/bin/sh".to_string(),
            "-c".to_string(),
            config_script.to_string(),
        ]),
        env: Some(init_env),
        volume_mounts: Some(vec![VolumeMount {
            name: "config".to_string(),
            mount_path: "/config".to_string(),
            ..Default::default()
        }]),
        ..Default::default()
    });

    // Add user-defined init container if specified
    if let Some(ref init_config) = instance.spec.init_container {
        let mut init_env = Vec::new();
        for env in &init_config.env {
            if let Some(ref value) = env.value {
                init_env.push(EnvVar {
                    name: env.name.clone(),
                    value: Some(value.clone()),
                    ..Default::default()
                });
            }
        }

        init_containers.push(Container {
            name: "init-config".to_string(),
            image: Some(init_config.image.clone()),
            command: if init_config.command.is_empty() {
                None
            } else {
                Some(init_config.command.clone())
            },
            args: if init_config.args.is_empty() {
                None
            } else {
                Some(init_config.args.clone())
            },
            env: if init_env.is_empty() {
                None
            } else {
                Some(init_env)
            },
            volume_mounts: Some(vec![VolumeMount {
                name: "config".to_string(),
                mount_path: "/config".to_string(),
                ..Default::default()
            }]),
            ..Default::default()
        });
    }

    // Build main container
    let container = Container {
        name: "sonarr".to_string(),
        image: Some(instance.spec.image.clone()),
        image_pull_policy: Some(instance.spec.image_pull_policy.clone()),
        ports: Some(vec![ContainerPort {
            container_port: instance.container_port(),
            name: Some("http".to_string()),
            protocol: Some("TCP".to_string()),
            ..Default::default()
        }]),
        env: Some(env_vars),
        volume_mounts: Some(volume_mounts),
        resources: instance
            .spec
            .resources
            .as_ref()
            .map(|r| ResourceRequirements {
                limits: if r.limits.is_empty() {
                    None
                } else {
                    Some(
                        r.limits
                            .iter()
                            .map(|(k, v)| (k.clone(), Quantity(v.clone())))
                            .collect(),
                    )
                },
                requests: if r.requests.is_empty() {
                    None
                } else {
                    Some(
                        r.requests
                            .iter()
                            .map(|(k, v)| (k.clone(), Quantity(v.clone())))
                            .collect(),
                    )
                },
                ..Default::default()
            }),
        liveness_probe: Some(k8s_openapi::api::core::v1::Probe {
            http_get: Some(k8s_openapi::api::core::v1::HTTPGetAction {
                path: Some("/ping".to_string()),
                port: k8s_openapi::apimachinery::pkg::util::intstr::IntOrString::Int(
                    instance.container_port(),
                ),
                ..Default::default()
            }),
            initial_delay_seconds: Some(60),
            period_seconds: Some(30),
            failure_threshold: Some(5),
            ..Default::default()
        }),
        readiness_probe: Some(k8s_openapi::api::core::v1::Probe {
            http_get: Some(k8s_openapi::api::core::v1::HTTPGetAction {
                path: Some("/ping".to_string()),
                port: k8s_openapi::apimachinery::pkg::util::intstr::IntOrString::Int(
                    instance.container_port(),
                ),
                ..Default::default()
            }),
            initial_delay_seconds: Some(10),
            period_seconds: Some(10),
            ..Default::default()
        }),
        ..Default::default()
    };

    // Build pod security context
    let pod_security_context = instance.spec.security_context.as_ref().map(|sc| {
        k8s_openapi::api::core::v1::PodSecurityContext {
            run_as_user: sc.run_as_user,
            run_as_group: sc.run_as_group,
            fs_group: sc.fs_group,
            run_as_non_root: sc.run_as_non_root,
            ..Default::default()
        }
    });

    // Build tolerations
    let tolerations: Option<Vec<k8s_openapi::api::core::v1::Toleration>> =
        if instance.spec.tolerations.is_empty() {
            None
        } else {
            Some(
                instance
                    .spec
                    .tolerations
                    .iter()
                    .map(|t| k8s_openapi::api::core::v1::Toleration {
                        key: t.key.clone(),
                        operator: t.operator.clone(),
                        value: t.value.clone(),
                        effect: t.effect.clone(),
                        toleration_seconds: t.toleration_seconds,
                    })
                    .collect(),
            )
        };

    let deployment = Deployment {
        metadata: ObjectMeta {
            name: Some(deployment_name.clone()),
            namespace: Some(namespace.to_string()),
            labels: Some(labels.clone()),
            owner_references: Some(vec![owner_ref.clone()]),
            ..Default::default()
        },
        spec: Some(DeploymentSpec {
            replicas: Some(instance.spec.replicas),
            selector: LabelSelector {
                match_labels: Some(labels.clone()),
                ..Default::default()
            },
            template: PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    labels: Some(labels.clone()),
                    ..Default::default()
                }),
                spec: Some(PodSpec {
                    init_containers: if init_containers.is_empty() {
                        None
                    } else {
                        Some(init_containers)
                    },
                    containers: vec![container],
                    volumes: Some(volumes),
                    node_selector: if instance.spec.node_selector.is_empty() {
                        None
                    } else {
                        Some(instance.spec.node_selector.clone())
                    },
                    tolerations,
                    security_context: pod_security_context,
                    ..Default::default()
                }),
            },
            ..Default::default()
        }),
        ..Default::default()
    };

    Ok(deployment)
}

async fn reconcile_service(
    client: &Client,
    namespace: &str,
    instance: &Sonarr,
    owner_ref: &OwnerReference,
    labels: &BTreeMap<String, String>,
) -> Result<()> {
    let service_name = instance.service_name();
    let service_api: Api<Service> = Api::namespaced(client.clone(), namespace);

    let service = Service {
        metadata: ObjectMeta {
            name: Some(service_name.clone()),
            namespace: Some(namespace.to_string()),
            labels: Some(labels.clone()),
            annotations: if instance.spec.service.annotations.is_empty() {
                None
            } else {
                Some(instance.spec.service.annotations.clone())
            },
            owner_references: Some(vec![owner_ref.clone()]),
            ..Default::default()
        },
        spec: Some(ServiceSpec {
            type_: Some(instance.service_type()),
            selector: Some(labels.clone()),
            ports: Some(vec![ServicePort {
                name: Some("http".to_string()),
                port: instance.service_port(),
                target_port: Some(
                    k8s_openapi::apimachinery::pkg::util::intstr::IntOrString::Int(
                        instance.container_port(),
                    ),
                ),
                node_port: instance.spec.service.node_port,
                protocol: Some("TCP".to_string()),
                ..Default::default()
            }]),
            ..Default::default()
        }),
        ..Default::default()
    };

    let patch_params = PatchParams::apply("sonarr-operator").force();
    service_api
        .patch(&service_name, &patch_params, &Patch::Apply(&service))
        .await
        .map_err(Error::KubeError)?;

    info!("Service {} applied", service_name);

    Ok(())
}

async fn reconcile_ingress(
    client: &Client,
    namespace: &str,
    instance: &Sonarr,
    ingress_config: &crate::crds::sonarr::IngressConfig,
    owner_ref: &OwnerReference,
    labels: &BTreeMap<String, String>,
) -> Result<()> {
    let ingress_name = format!("{}-sonarr", instance.name_any());
    let ingress_api: Api<Ingress> = Api::namespaced(client.clone(), namespace);

    let tls = ingress_config.tls.as_ref().map(|tls_config| {
        vec![IngressTLS {
            hosts: Some(if tls_config.hosts.is_empty() {
                vec![ingress_config.host.clone()]
            } else {
                tls_config.hosts.clone()
            }),
            secret_name: Some(tls_config.secret_name.clone()),
        }]
    });

    let ingress = Ingress {
        metadata: ObjectMeta {
            name: Some(ingress_name.clone()),
            namespace: Some(namespace.to_string()),
            labels: Some(labels.clone()),
            annotations: if ingress_config.annotations.is_empty() {
                None
            } else {
                Some(ingress_config.annotations.clone())
            },
            owner_references: Some(vec![owner_ref.clone()]),
            ..Default::default()
        },
        spec: Some(IngressSpec {
            ingress_class_name: ingress_config.ingress_class_name.clone(),
            tls,
            rules: Some(vec![IngressRule {
                host: Some(ingress_config.host.clone()),
                http: Some(HTTPIngressRuleValue {
                    paths: vec![HTTPIngressPath {
                        path: Some(ingress_config.path.clone()),
                        path_type: ingress_config.path_type.clone(),
                        backend: IngressBackend {
                            service: Some(IngressServiceBackend {
                                name: instance.service_name(),
                                port: Some(ServiceBackendPort {
                                    number: Some(instance.service_port()),
                                    ..Default::default()
                                }),
                            }),
                            ..Default::default()
                        },
                    }],
                }),
            }]),
            ..Default::default()
        }),
        ..Default::default()
    };

    let patch_params = PatchParams::apply("sonarr-operator").force();
    ingress_api
        .patch(&ingress_name, &patch_params, &Patch::Apply(&ingress))
        .await
        .map_err(Error::KubeError)?;

    info!("Ingress {} applied", ingress_name);

    Ok(())
}

async fn reconcile_http_route(
    client: &Client,
    namespace: &str,
    instance: &Sonarr,
    http_route_config: &crate::crds::sonarr::HTTPRouteConfig,
    owner_ref: &OwnerReference,
    labels: &BTreeMap<String, String>,
) -> Result<()> {
    use kube::api::DynamicObject;
    use kube::discovery::ApiResource;

    let route_name = format!("{}-sonarr", instance.name_any());

    // Build the HTTPRoute as a DynamicObject since gateway.networking.k8s.io types
    // are not in k8s-openapi yet
    let api_resource = ApiResource {
        group: "gateway.networking.k8s.io".to_string(),
        version: "v1".to_string(),
        kind: "HTTPRoute".to_string(),
        api_version: "gateway.networking.k8s.io/v1".to_string(),
        plural: "httproutes".to_string(),
    };

    let route_api: Api<DynamicObject> =
        Api::namespaced_with(client.clone(), namespace, &api_resource);

    // Merge labels
    let mut route_labels = labels.clone();
    for (k, v) in &http_route_config.labels {
        route_labels.insert(k.clone(), v.clone());
    }

    // Build parent reference
    let mut parent_ref = serde_json::json!({
        "group": "gateway.networking.k8s.io",
        "kind": "Gateway",
        "name": http_route_config.gateway_ref.name
    });

    if let Some(ref ns) = http_route_config.gateway_ref.namespace {
        parent_ref["namespace"] = serde_json::json!(ns);
    }
    if let Some(ref section) = http_route_config.gateway_ref.section_name {
        parent_ref["sectionName"] = serde_json::json!(section);
    }

    // Build path match
    let path_match = serde_json::json!({
        "type": http_route_config.path_type,
        "value": http_route_config.path
    });

    // Build backend ref
    let backend_ref = serde_json::json!({
        "kind": "Service",
        "name": instance.service_name(),
        "port": instance.service_port()
    });

    // Build the HTTPRoute spec
    let http_route_spec = serde_json::json!({
        "parentRefs": [parent_ref],
        "hostnames": if http_route_config.hostnames.is_empty() { None } else { Some(&http_route_config.hostnames) },
        "rules": [{
            "matches": [{
                "path": path_match
            }],
            "backendRefs": [backend_ref]
        }]
    });

    // Build the full HTTPRoute object
    let http_route = serde_json::json!({
        "apiVersion": "gateway.networking.k8s.io/v1",
        "kind": "HTTPRoute",
        "metadata": {
            "name": route_name,
            "namespace": namespace,
            "labels": route_labels,
            "annotations": if http_route_config.annotations.is_empty() { None } else { Some(&http_route_config.annotations) },
            "ownerReferences": [owner_ref]
        },
        "spec": http_route_spec
    });

    let patch_params = PatchParams::apply("sonarr-operator").force();
    route_api
        .patch(&route_name, &patch_params, &Patch::Apply(&http_route))
        .await
        .map_err(Error::KubeError)?;

    info!("HTTPRoute {} applied", route_name);

    Ok(())
}

async fn update_status(client: &Client, namespace: &str, instance: &Sonarr) -> Result<()> {
    let instances: Api<Sonarr> = Api::namespaced(client.clone(), namespace);
    let name = instance.name_any();

    // Check deployment status
    let deployment_api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    let deployment_name = instance.deployment_name();

    let (ready, ready_replicas) = match deployment_api.get_opt(&deployment_name).await? {
        Some(deployment) => {
            let status = deployment.status.unwrap_or_default();
            let ready = status.ready_replicas.unwrap_or(0);
            let desired = status.replicas.unwrap_or(0);
            (ready >= desired && desired > 0, ready)
        }
        None => (false, 0),
    };

    // Build URL
    let url = if let Some(ref ingress) = instance.spec.ingress {
        if ingress.enabled {
            let scheme = if ingress.tls.is_some() {
                "https"
            } else {
                "http"
            };
            Some(format!("{}://{}{}", scheme, ingress.host, ingress.path))
        } else {
            Some(instance.internal_url(namespace))
        }
    } else {
        Some(instance.internal_url(namespace))
    };

    // Build conditions
    let mut conditions = instance
        .status
        .as_ref()
        .map(|s| s.conditions.clone())
        .unwrap_or_default();

    if ready {
        update_conditions(
            &mut conditions,
            ready_condition(true, "Ready", "Sonarr instance is ready"),
        );
    } else {
        update_conditions(
            &mut conditions,
            ready_condition(false, "NotReady", "Sonarr instance is not ready"),
        );
        update_conditions(
            &mut conditions,
            progressing_condition(true, "Deploying", "Deployment is in progress"),
        );
    }

    // Get API key secret name
    let api_key_secret = if instance.spec.api_key_secret_ref.is_some() {
        instance
            .spec
            .api_key_secret_ref
            .as_ref()
            .map(|s| s.name.clone())
    } else {
        Some(instance.api_key_secret_name())
    };

    let status = SonarrStatus {
        conditions,
        url,
        api_key_secret,
        observed_generation: instance.metadata.generation.unwrap_or(0),
        ready_replicas,
        version: None, // TODO: Get from Sonarr API
    };

    let status_patch = serde_json::json!({
        "status": status
    });

    instances
        .patch_status(&name, &PatchParams::default(), &Patch::Merge(&status_patch))
        .await
        .map_err(Error::KubeError)?;

    Ok(())
}
