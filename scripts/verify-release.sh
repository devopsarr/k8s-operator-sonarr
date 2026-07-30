#!/usr/bin/env bash
# Verify a published release of the sonarr-operator:
#   - operator image exists on GHCR for the given version
#   - chart exists on GHCR
#   - cosign signatures verify (keyless, GitHub OIDC)
#   - chart renders cleanly with helm template
#
# Usage:
#   scripts/verify-release.sh v0.1.0
#
# Requirements: docker, cosign, helm, jq (optional)
# For private packages, run `docker login ghcr.io` and
# `helm registry login ghcr.io` beforehand.

set -euo pipefail

VERSION="${1:?usage: $0 <version> (e.g. v0.1.0)}"
VERSION_NOPREFIX="${VERSION#v}"

OWNER="${OWNER:-devopsarr}"
REPO="${REPO:-k8s-operator-sonarr}"
IMAGE="ghcr.io/${OWNER}/${REPO}:${VERSION}"
CHART_REF="ghcr.io/${OWNER}/charts/sonarr-operator"
CHART="oci://${CHART_REF}"
EXPECTED_IDENTITY_REGEX="^https://github.com/${OWNER}/${REPO}/"
OIDC_ISSUER="https://token.actions.githubusercontent.com"

step() { printf "\n==> %s\n" "$*"; }
ok()   { printf "  ✓ %s\n" "$*"; }
fail() { printf "  ✗ %s\n" "$*" >&2; exit 1; }

require() {
  command -v "$1" >/dev/null 2>&1 || fail "missing required tool: $1"
}

require docker
require cosign
require helm

step "Pulling operator image: ${IMAGE}"
docker pull "${IMAGE}" >/dev/null && ok "image pulled"

step "Verifying image signature (keyless cosign)"
cosign verify "${IMAGE}" \
  --certificate-identity-regexp "${EXPECTED_IDENTITY_REGEX}" \
  --certificate-oidc-issuer "${OIDC_ISSUER}" >/dev/null && ok "image signature verified"

step "Pulling chart: ${CHART} --version ${VERSION_NOPREFIX}"
WORKDIR="$(mktemp -d)"
trap 'rm -rf "${WORKDIR}"' EXIT
helm pull "${CHART}" --version "${VERSION_NOPREFIX}" --destination "${WORKDIR}" >/dev/null && ok "chart pulled"

CHART_TGZ="$(ls "${WORKDIR}"/sonarr-operator-*.tgz)"
ok "chart artifact: $(basename "${CHART_TGZ}")"

step "Verifying chart signature (keyless cosign)"
cosign verify "${CHART_REF}:${VERSION_NOPREFIX}" \
  --certificate-identity-regexp "${EXPECTED_IDENTITY_REGEX}" \
  --certificate-oidc-issuer "${OIDC_ISSUER}" >/dev/null && ok "chart signature verified"

step "helm template (defaults)"
helm template verify "${CHART_TGZ}" --namespace sonarr-operator-system >/dev/null && ok "chart renders"

step "helm template (CRDs gated out)"
helm template verify "${CHART_TGZ}" --namespace sonarr-operator-system \
  --set crds.install=false >/dev/null && ok "chart renders without CRDs"

step "All checks passed for ${VERSION}"
