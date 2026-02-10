# Testing the Sonarr Kubernetes Operator

This guide covers different approaches to testing the operator, from unit tests to full end-to-end testing on Kubernetes.

## Prerequisites

- Rust toolchain (1.75+)
- Docker
- kubectl
- [Kind](https://kind.sigs.k8s.io/) (for local Kubernetes testing)
- A Sonarr instance (for integration tests)

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

Run unit tests with:

```bash
make test
# or with verbose output
make test-verbose
```

Unit tests are located in each module and test individual functions without external dependencies.

---

## 2. Local Development Testing

### Run Operator Locally

You can run the operator locally against any Kubernetes cluster:

```bash
# Ensure you have a valid kubeconfig
export KUBECONFIG=~/.kube/config

# Install CRDs first
make install

# Run the operator with debug logging
make run-debug
```

The operator will connect to your cluster and start reconciling Sonarr resources.

---

## 3. Integration Testing with Kind

[Kind](https://kind.sigs.k8s.io/) provides a local Kubernetes cluster using Docker containers.

### Setup Kind Cluster

```bash
# Create a Kind cluster
make kind-create

# Install CRDs
make install
```

### Deploy Test Sonarr Instance

```bash
# Deploy a real Sonarr instance for testing
kubectl apply -f tests/e2e/fixtures/sonarr-deployment.yaml

# Wait for Sonarr to be ready
kubectl wait --for=condition=available deployment/sonarr -n sonarr-test --timeout=120s

# Port-forward to access Sonarr UI (optional)
kubectl port-forward -n sonarr-test svc/sonarr 8989:8989
```

### Get Sonarr API Key

```bash
# Wait for Sonarr to initialize (creates config.xml)
sleep 30

# Extract API key from Sonarr config
kubectl exec -n sonarr-test deployment/sonarr -- cat /config/config.xml | grep -oP '(?<=<ApiKey>)[^<]+'
```

### Update API Key Secret

Edit `tests/e2e/fixtures/01-sonarr-instance.yaml` and replace the placeholder API key with the actual value:

```bash
# Or create the secret directly
kubectl create secret generic sonarr-api-key -n sonarr-test \
  --from-literal=api-key=YOUR_ACTUAL_API_KEY \
  --dry-run=client -o yaml | kubectl apply -f -
```

### Run Operator and Test Resources

```bash
# In terminal 1: Run the operator
make run-debug

# In terminal 2: Apply test resources
kubectl apply -f tests/e2e/fixtures/01-sonarr-instance.yaml
kubectl apply -f tests/e2e/fixtures/02-tags.yaml
kubectl apply -f tests/e2e/fixtures/03-root-folders.yaml

# Check the resources
kubectl get sonarrs,sonarrtags,sonarrrootfolders -n sonarr-test -o wide

# Check status of a specific resource
kubectl describe sonarrtag test-tag-anime -n sonarr-test
```

### Verify in Sonarr

Access the Sonarr UI at http://localhost:8989 (if port-forwarded) and verify:
- Tags appear in Settings → Tags
- Root folders appear in Settings → Media Management

### Cleanup

```bash
make kind-delete
```

---

## 4. Full E2E Testing with Docker Image

Build and deploy the operator as a container:

```bash
# Create Kind cluster
make kind-create

# Build Docker image
make docker

# Load image into Kind
make kind-load

# Deploy operator to cluster
make deploy

# Apply test fixtures
kubectl apply -f tests/e2e/fixtures/

# Watch resources
kubectl get sonarrs,sonarrtags -n sonarr-test -w
```

---

## 5. Testing Individual Components

### Test CRD Generation

```bash
# Generate all CRDs
make crds
cat deploy/crds/crds.yaml

# Generate single CRD
make crds-single CRD=SonarrTag
```

### Test Specific Resource Types

```bash
# Apply and test tags
kubectl apply -f - <<EOF
apiVersion: devopsarr.io/v1alpha1
kind: SonarrTag
metadata:
  name: my-test-tag
  namespace: sonarr-test
spec:
  sonarrInstanceRef:
    name: test-sonarr
  label: "my-tag"
EOF

# Check status
kubectl get sonarrtag my-test-tag -n sonarr-test -o yaml

# Delete and verify cleanup
kubectl delete sonarrtag my-test-tag -n sonarr-test
```

---

## 6. Debugging Tips

### Check Operator Logs

```bash
# If running locally
RUST_LOG=debug cargo run --bin sonarr-operator

# If running in cluster
kubectl logs -n sonarr-operator-system deployment/sonarr-operator -f
```

### Check Resource Events

```bash
kubectl describe sonarrtag test-tag-anime -n sonarr-test
# Look for "Events" section at the bottom
```

### Verify Sonarr API Connectivity

```bash
# Port-forward to Sonarr
kubectl port-forward -n sonarr-test svc/sonarr 8989:8989

# Test API (replace with actual API key)
curl -H "X-Api-Key: YOUR_API_KEY" http://localhost:8989/api/v3/tag
```

### Common Issues

1. **API Key not found**: Ensure the secret exists and has the correct key name
2. **Sonarr not ready**: Check if Sonarr deployment is running and healthy
3. **CRD not installed**: Run `make install` to apply CRDs
4. **Finalizer stuck**: Check operator logs for errors during cleanup

---

## 7. CI/CD Integration

For CI pipelines, use:

```bash
# Run all checks
make ci

# Full test including build
make all
```

Example GitHub Actions workflow:

```yaml
name: CI
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-action@stable
      - run: make lint test build crds
```

---

## 8. Test Coverage

Generate test coverage report:

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
open tarpaulin-report.html
```
