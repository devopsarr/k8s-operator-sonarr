# Testing the Sonarr Kubernetes Operator

This guide covers different approaches to testing the operator, from unit tests to full end-to-end testing on Kubernetes.

## Prerequisites

- Rust toolchain (1.88+)
- Docker
- kubectl
- [k3d](https://k3d.io/) or [Kind](https://kind.sigs.k8s.io/) for local Kubernetes clusters

## Quick Start

```bash
# Run linting and unit tests
make lint test

# Build the operator
make build

# Generate CRDs
make crds
```

---

## 1. Unit Testing

```bash
make test
# or with verbose output
make test-verbose
```

Unit tests are located in each module and test individual functions without external dependencies.

---

## 2. Integration Testing

Integration tests verify CRD schemas and validation against a live Kubernetes cluster (no Sonarr instance required).

```bash
# Setup
make kind-create install

# Run
cargo test --test integration -- --ignored --test-threads=1

# Cleanup
make kind-delete
```

---

## 3. End-to-End Testing

E2E tests verify the full reconciliation loop: the operator watches CRDs, calls the Sonarr API, and updates resource status.

### Architecture

The E2E flow mirrors the CI pipeline:

1. Create a Kubernetes cluster and install CRDs
2. Run the operator (locally or as a Deployment)
3. Create an API key Secret and apply the **Sonarr CR**
4. The operator reconciles the Sonarr CR → creates Deployment, Service, PVC
5. Sonarr boots with the deterministic API key
6. Tests create sub-resource CRs (Tags, RootFolders, etc.) and verify via the Sonarr API

### Local Setup

```bash
# Create a k3d cluster with NodePort mapping (host 8989 → node 30989)
k3d cluster create sonarr-e2e --port 8989:30989@server:0 --no-lb

# Install CRDs
make install

# Run the operator (terminal 1)
make run-debug

# Deploy Sonarr via its CRD (terminal 2)
export SONARR_API_KEY="my-test-api-key"
make e2e-deploy-sonarr

# Run the E2E tests
export SONARR_URL="http://localhost:8989"
make e2e
```

### Fixture Files

The only fixture file is:

- **`tests/e2e/fixtures/sonarr-instance.yaml`** — the Sonarr CR that the operator reconciles into a running Sonarr instance with `serviceType: NodePort` on port 30989.

All test resources (Tags, RootFolders, QualityProfiles, etc.) are created programmatically by the Rust test code.

### Running E2E Tests

```bash
# Standard run
make e2e

# Verbose with debug logging
make e2e-verbose
```

### Cleanup

```bash
make e2e-cleanup
k3d cluster delete sonarr-e2e
```

---

## 4. CI Pipeline

The GitHub Actions CI pipeline automates the full E2E flow:

1. **Build** the operator image and push to GHCR
2. **Create** a k3d cluster with NodePort mapping
3. **Deploy** the operator as a Kubernetes Deployment (from the GHCR image)
4. **Apply** the Sonarr CR with a deterministic API key
5. **Run** E2E tests against the live Sonarr instance

See `.github/workflows/ci.yml` for the full pipeline definition.

---

## 5. Testing Individual Components

### Test CRD Generation

```bash
make crds
ls deploy/crds/

# Generate a single CRD
make crds-single CRD=SonarrTag
```

### Test a Specific Resource

```bash
kubectl apply -f - <<EOF
apiVersion: devopsarr.io/v1alpha1
kind: SonarrTag
metadata:
  name: my-test-tag
  namespace: sonarr-e2e-test
spec:
  sonarrInstanceRef:
    name: sonarr
    namespace: default
  label: "my-tag"
EOF

kubectl get sonarrtag my-test-tag -n sonarr-e2e-test -o yaml
kubectl delete sonarrtag my-test-tag -n sonarr-e2e-test
```

---

## 6. Debugging Tips

### Check Operator Logs

```bash
# Local
RUST_LOG=debug cargo run --bin sonarr-operator

# In-cluster
kubectl logs -n sonarr-operator-system deployment/sonarr-operator -f
```

### Verify Sonarr API

```bash
curl -H "X-Api-Key: $SONARR_API_KEY" http://localhost:8989/api/v3/tag
curl -H "X-Api-Key: $SONARR_API_KEY" http://localhost:8989/api/v3/system/status
```

### Common Issues

| Symptom | Fix |
|---------|-----|
| `SONARR_API_KEY not set` | `export SONARR_API_KEY=<your-key>` |
| Sonarr CR never becomes Ready | Check operator logs: `kubectl logs -n sonarr-operator-system deploy/sonarr-operator` |
| CRD not found | Run `make install` to apply CRDs |
| Finalizer stuck on delete | Check operator logs for cleanup errors |
| NodePort not reachable | Ensure k3d was created with `--port 8989:30989@server:0` |

---

## 7. Test Coverage

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
open tarpaulin-report.html
```
