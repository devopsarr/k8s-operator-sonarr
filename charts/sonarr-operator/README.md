# sonarr-operator

A Kubernetes operator that manages [Sonarr](https://sonarr.tv/) instances and their configuration declaratively through Custom Resources.

This chart installs the operator Deployment, RBAC, and (optionally) the CRDs that the operator reconciles.

## TL;DR

```bash
helm install sonarr-operator \
  oci://ghcr.io/devopsarr/charts/sonarr-operator \
  --namespace sonarr-operator-system \
  --create-namespace
```

## Prerequisites

- Kubernetes >= 1.28
- Helm >= 3.8 (OCI support)

## Installing the chart

```bash
helm install sonarr-operator \
  oci://ghcr.io/devopsarr/charts/sonarr-operator \
  --version <version> \
  --namespace sonarr-operator-system \
  --create-namespace
```

The chart ships **19 CRDs** under the `devopsarr.io/v1alpha1` API group. They are installed by default and annotated `helm.sh/resource-policy: keep`, so they (and your `Sonarr` resources) survive a `helm uninstall`.

If you manage CRDs out-of-band (for example with a separate Flux/ArgoCD application), disable them:

```bash
helm install sonarr-operator oci://ghcr.io/devopsarr/charts/sonarr-operator \
  --set crds.install=false
```

To allow `helm uninstall` to remove CRDs as well, opt out of the `keep` policy:

```bash
helm install sonarr-operator oci://ghcr.io/devopsarr/charts/sonarr-operator \
  --set crds.keep=false
```

## Uninstalling the chart

```bash
helm uninstall sonarr-operator -n sonarr-operator-system
```

CRDs are intentionally **not** removed by `helm uninstall`. To purge them and all managed resources:

```bash
kubectl get crd -o name | grep devopsarr.io | xargs kubectl delete
```

## Values

See [`values.yaml`](./values.yaml) for the full set of values with inline documentation.

| Key | Default | Description |
|---|---|---|
| `replicaCount` | `1` | Number of operator replicas. |
| `image.repository` | `ghcr.io/devopsarr/k8s-operator-sonarr` | Operator image. |
| `image.tag` | `""` | Image tag. Defaults to `Chart.AppVersion`. |
| `image.pullPolicy` | `IfNotPresent` | Image pull policy. |
| `crds.install` | `true` | Install the operator's CRDs as part of the chart. |
| `crds.keep` | `true` | Annotate CRDs with `helm.sh/resource-policy: keep`. |
| `crds.annotations` | `{}` | Extra annotations merged into each CRD. |
| `crds.additionalLabels` | `{}` | Extra labels merged into each CRD. |
| `serviceAccount.create` | `true` | Create a ServiceAccount for the operator. |
| `rbac.create` | `true` | Create ClusterRole and ClusterRoleBinding. |
| `logLevel` | `info,sonarr_operator=debug` | `RUST_LOG` value. |
| `resources` | requests `50m`/`64Mi`, limits `200m`/`256Mi` | Container resources. |

## Source

- Operator: <https://github.com/devopsarr/k8s-operator-sonarr>
- CRD reference: <https://github.com/devopsarr/k8s-operator-sonarr/blob/main/docs/api/crd-reference.md>
