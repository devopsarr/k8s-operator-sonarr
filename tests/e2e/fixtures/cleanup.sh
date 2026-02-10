#!/bin/bash
# Cleanup script for E2E tests

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NAMESPACE="${NAMESPACE:-default}"

echo "=== Sonarr E2E Test Cleanup ==="

# Delete test resources
echo "Cleaning up test resources..."
kubectl delete -f "${SCRIPT_DIR}/sonarr-instance.yaml" --ignore-not-found
kubectl delete secret sonarr-api-key -n "${NAMESPACE}" --ignore-not-found

# Delete E2E test namespace
echo "Cleaning up E2E test namespace..."
kubectl delete namespace sonarr-e2e-test --ignore-not-found

# Delete Sonarr deployment
echo "Cleaning up Sonarr deployment..."
kubectl delete -f "${SCRIPT_DIR}/sonarr-deployment.yaml" --ignore-not-found

# Clean up local files
rm -f "${SCRIPT_DIR}/.sonarr-api-key"

echo "=== Cleanup Complete ==="
