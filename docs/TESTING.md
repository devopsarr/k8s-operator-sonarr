# Testing the Sonarr Kubernetes Operator

This guide covers the different testing approaches, from unit tests to full end-to-end testing on Kubernetes.

## Prerequisites

- Rust toolchain (1.88+)
- Docker
- kubectl
- [k3d](https://k3d.io/) for local Kubernetes clusters

## Quick Start

```bash
# Lint and unit tests
make lint test

# Build
make build

# Generate CRDs
make crds
```

---

## 1. Unit Testing

```bash
make test
```

Unit tests are located in each module and test individual functions without external dependencies.

---

## 2. Integration Testing

Integration tests verify CRD schemas and validation against a live Kubernetes cluster (no Sonarr instance required).

```bash
# Create a k3d cluster and install CRDs
make e2e-cluster-create
make install

# Run integration tests
make integration-test

# Cleanup
make e2e-cluster-delete
```

---

## 3. End-to-End Testing

E2E tests verify the full reconciliation loop: the operator watches CRDs, calls the Sonarr API, and updates resource status.

### Architecture

1. Create a k3d cluster with NodePort mapping and install CRDs
2. Run the operator locally (or as a Deployment)
3. Create an API key Secret and apply the **Sonarr CR**
4. The operator reconciles the Sonarr CR → creates Deployment, Service, PVC
5. Sonarr boots with the deterministic API key
6. Tests create sub-resource CRs (Tags, RootFolders, etc.) and verify via the Sonarr API

### Local Setup

```bash
# Terminal 1: Create cluster + deploy Sonarr
make e2e-up

# Terminal 2: Run the operator locally
make run-debug

# Terminal 3: Run E2E tests
make e2e

# Verbose with debug logging
make e2e-verbose
```

### Fixture Files

The only fixture file is:

- **`tests/e2e/fixtures/sonarr-instance.yaml`** — the Sonarr CR that the operator reconciles into a running Sonarr instance with `serviceType: NodePort` on port 30989.

All test resources (Tags, RootFolders, QualityProfiles, etc.) are created programmatically by the Rust test code.

### Cleanup

```bash
# Remove test resources (keep cluster)
make e2e-cleanup

# Tear down everything
make e2e-down
```

---

## 4. CI Pipeline

The GitHub Actions CI pipeline (`.github/workflows/ci.yml`) automates the full test suite:

1. **Lint & Format** — `cargo fmt --check` + `cargo clippy`
2. **Unit Tests** — `cargo test --lib`
3. **Generate CRDs** — `cargo run --bin crdgen`
4. **Build & Push Image** — Docker build + push to GHCR
5. **Integration Tests** — CRD schema validation on K8s v1.28, v1.29, and latest
6. **E2E Tests** — Full reconciliation against a live Sonarr instance in k3d

---

## 5. Debugging Tips

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
