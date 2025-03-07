# Rust Kubernetes Operator

A Kubernetes operator written in Rust using the kube-rs library.

## Prerequisites

- Rust 1.71+
- Kubernetes cluster (local or remote)
- kubectl configured to access your cluster

## Building and Running

### Local Development

```bash
# Build the project
cargo build

# Run the operator locally
cargo run

# Run with a specific namespace
cargo run -- --namespace=default
```

### Running in Kubernetes

```bash
# Apply the CRD
kubectl apply -f manifests/crd.yaml

# Build and push the Docker image
docker build -t your-registry/rust-k8s-operator:latest .
docker push your-registry/rust-k8s-operator:latest

# Deploy the operator (update the image reference in the deployment manifest)
kubectl apply -f manifests/deployment.yaml

# Create an example resource
kubectl apply -f manifests/example-cr.yaml
```


## CRD Generation

This project uses code-first CRD definition. The CRD YAML is generated from the Rust code to ensure they stay in sync.

### Generating the CRD

```bash
# Using cargo directly
cargo run --bin generate_crd

# Or using the provided shell script
./generate-crd.sh

# Generate and apply to the cluster
./generate-crd.sh --apply
```

The generated CRD will be saved to `manifests/crd.yaml`.

### How it works

The CRD is defined in the Rust code using the `#[derive(CustomResource)]` macro from the kube-rs crate. The `generate_crd` binary uses the `CustomResourceExt` trait to generate the YAML representation of the CRD.

If you modify the CRD definition in the Rust code, make sure to regenerate the YAML manifest.

## Project Structure

- `src/main.rs`: Entry point for the operator
- `src/crd/`: Custom resource definitions
- `src/controller/`: Reconciliation logic
- `src/error/`: Error types
- `manifests/`: Kubernetes manifests for deployment

## License

MIT