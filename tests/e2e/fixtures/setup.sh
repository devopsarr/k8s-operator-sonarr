#!/bin/bash
# Setup script for E2E tests
# This script:
# 1. Deploys Sonarr
# 2. Waits for it to be ready
# 3. Extracts the API key
# 4. Creates the Sonarr CRD instance

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NAMESPACE="${NAMESPACE:-default}"
TIMEOUT="${TIMEOUT:-300}"

echo "=== Sonarr E2E Test Setup ==="

# Step 1: Deploy Sonarr
echo "Step 1: Deploying Sonarr..."
kubectl apply -f "${SCRIPT_DIR}/sonarr-deployment.yaml"

# Step 2: Wait for Sonarr to be ready
echo "Step 2: Waiting for Sonarr deployment to be ready..."
kubectl wait --for=condition=available deployment/sonarr -n "${NAMESPACE}" --timeout="${TIMEOUT}s"

echo "Waiting for Sonarr pod to be ready..."
kubectl wait --for=condition=ready pod -l app=sonarr -n "${NAMESPACE}" --timeout="${TIMEOUT}s"

# Give Sonarr time to initialize and generate API key
echo "Waiting for Sonarr to initialize..."
sleep 30

# Step 3: Extract API key from Sonarr config
echo "Step 3: Extracting API key from Sonarr..."
SONARR_POD=$(kubectl get pod -l app=sonarr -n "${NAMESPACE}" -o jsonpath='{.items[0].metadata.name}')

# Try to get API key from config file
API_KEY=""
for i in {1..10}; do
    API_KEY=$(kubectl exec "${SONARR_POD}" -n "${NAMESPACE}" -- cat /config/config.xml 2>/dev/null | grep -oP '(?<=<ApiKey>)[^<]+' || true)
    if [ -n "${API_KEY}" ]; then
        break
    fi
    echo "Waiting for API key to be generated... (attempt $i/10)"
    sleep 10
done

if [ -z "${API_KEY}" ]; then
    echo "ERROR: Failed to extract API key from Sonarr"
    echo "Trying alternative method..."

    # Try to get it from the API directly (requires port-forward)
    kubectl port-forward "pod/${SONARR_POD}" 8989:8989 -n "${NAMESPACE}" &
    PF_PID=$!
    sleep 5

    # Check if Sonarr is responding
    curl -s http://localhost:8989/api/v3/system/status || true

    kill $PF_PID 2>/dev/null || true

    echo "Please check Sonarr logs for issues:"
    kubectl logs "${SONARR_POD}" -n "${NAMESPACE}" --tail=50
    exit 1
fi

echo "API key extracted successfully"

# Step 4: Update the secret with the real API key
echo "Step 4: Creating API key secret..."
kubectl create secret generic sonarr-api-key \
    --from-literal=api-key="${API_KEY}" \
    -n "${NAMESPACE}" \
    --dry-run=client -o yaml | kubectl apply -f -

# Step 5: Create the Sonarr CRD instance
echo "Step 5: Creating Sonarr CRD instance..."
kubectl apply -f "${SCRIPT_DIR}/sonarr-instance.yaml"

# Step 6: Export environment variables for tests
echo ""
echo "=== Setup Complete ==="
echo ""
echo "Sonarr is ready for E2E tests!"
echo ""
echo "To run E2E tests locally, set these environment variables:"
echo ""
echo "  export SONARR_API_KEY='${API_KEY}'"
echo "  export SONARR_URL='http://localhost:8989'"
echo ""
echo "Then port-forward Sonarr:"
echo "  kubectl port-forward svc/sonarr 8989:8989 -n ${NAMESPACE}"
echo ""
echo "And run the operator:"
echo "  cargo run"
echo ""
echo "Finally, run E2E tests:"
echo "  cargo test --test e2e -- --ignored --test-threads=1"
echo ""

# Create a file with the API key for CI usage
echo "${API_KEY}" > "${SCRIPT_DIR}/.sonarr-api-key"
echo "API key saved to ${SCRIPT_DIR}/.sonarr-api-key"
