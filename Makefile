# Sonarr Kubernetes Operator Makefile
#
# Usage:
#   make help           - Show this help
#   make build          - Build release binary
#   make crds           - Generate CRD manifests
#   make test           - Run unit tests
#   make e2e-up         - Full local E2E environment setup
#   make e2e            - Run E2E tests
#   make e2e-down       - Tear down local E2E environment

# Configuration
BINARY_NAME := sonarr-operator
CRD_DIR := deploy/crds
NAMESPACE := sonarr-operator-system
K3D_CLUSTER := sonarr-e2e
E2E_API_KEY ?= test-e2e-api-key-12345
SONARR_URL ?= http://localhost:8989

.PHONY: help
help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

##@ Development

.PHONY: check
check: ## Run cargo check
	cargo check

.PHONY: fmt
fmt: ## Format code
	cargo fmt

.PHONY: fmt-check
fmt-check: ## Check code formatting
	cargo fmt -- --check

.PHONY: clippy
clippy: ## Run clippy linter
	cargo clippy -- -D warnings

.PHONY: lint
lint: fmt-check clippy ## Run all linters

##@ Build

.PHONY: build
build: ## Build release binary
	cargo build --release

.PHONY: build-debug
build-debug: ## Build debug binary
	cargo build

.PHONY: clean
clean: ## Clean build artifacts
	cargo clean

##@ Testing

.PHONY: test
test: ## Run unit tests
	cargo test --lib

.PHONY: test-all
test-all: ## Run all non-ignored tests
	cargo test

.PHONY: integration-test
integration-test: install ## Run integration tests (requires cluster with CRDs)
	cargo test --test integration -- --ignored --test-threads=1

.PHONY: integration-test-verbose
integration-test-verbose: install ## Run integration tests with verbose output
	cargo test --test integration -- --ignored --test-threads=1 --nocapture

##@ CRD Management

.PHONY: crds
crds: ## Generate CRD manifests
	@mkdir -p $(CRD_DIR)
	@rm -f $(CRD_DIR)/*.yaml
	cargo run --bin crdgen -- --split $(CRD_DIR)

.PHONY: install
install: crds ## Install CRDs to cluster
	kubectl apply -f $(CRD_DIR)/

.PHONY: uninstall
uninstall: ## Remove CRDs from cluster
	kubectl delete -f $(CRD_DIR)/ --ignore-not-found

##@ Documentation

DOCS_DIR := docs/api
GOBIN := $(shell go env GOPATH)/bin

.PHONY: docs
docs: crds ## Generate CRD documentation
	@mkdir -p $(DOCS_DIR)
	@if ! command -v crdoc >/dev/null 2>&1 && [ ! -f "$(GOBIN)/crdoc" ]; then \
		echo "Installing crdoc..."; \
		go install fybrik.io/crdoc@latest; \
	fi
	@PATH="$(GOBIN):$$PATH" crdoc --resources $(CRD_DIR) --output $(DOCS_DIR)/crd-reference.md
	@echo "Documentation generated in $(DOCS_DIR)/crd-reference.md"

##@ Running

.PHONY: run
run: ## Run operator locally (requires kubeconfig)
	RUST_LOG=info cargo run --bin sonarr-operator

.PHONY: run-debug
run-debug: ## Run operator with debug logging
	RUST_LOG=debug cargo run --bin sonarr-operator

##@ Docker

.PHONY: docker
docker: ## Build Docker image and import into k3d cluster
	docker build -t $(BINARY_NAME):latest .
	k3d image import $(BINARY_NAME):latest --cluster $(K3D_CLUSTER)

##@ Kubernetes Deployment

.PHONY: deploy
deploy: install docker ## Deploy operator to cluster
	kubectl apply -f deploy/namespace.yaml
	kubectl apply -f deploy/rbac.yaml
	sed 's|image: .*|image: $(BINARY_NAME):latest|' deploy/deployment.yaml | kubectl apply -f -

.PHONY: undeploy
undeploy: ## Remove operator from cluster
	kubectl delete -f deploy/deployment.yaml --ignore-not-found
	kubectl delete -f deploy/rbac.yaml --ignore-not-found
	kubectl delete -f deploy/namespace.yaml --ignore-not-found

##@ Local E2E Testing (k3d)
#
# Full local E2E workflow:
#   1. make e2e-up        - Create k3d cluster, install CRDs, deploy Sonarr
#   2. make run-debug     - Run operator locally (in a separate terminal)
#   3. make e2e           - Run E2E tests
#   4. make e2e-down      - Tear down k3d cluster

.PHONY: e2e-cluster-create
e2e-cluster-create: ## Create k3d cluster with port mapping (8989 -> 30989)
	k3d cluster create $(K3D_CLUSTER) \
		--port 8989:30989@server:0 \
		--k3s-arg '--disable=traefik,servicelb,metrics-server@server:*'

.PHONY: e2e-cluster-delete
e2e-cluster-delete: ## Delete k3d cluster
	k3d cluster delete $(K3D_CLUSTER)

.PHONY: e2e-deploy-sonarr
e2e-deploy-sonarr: install ## Create API key secret + apply Sonarr CR
	kubectl create secret generic sonarr-api-key \
		--from-literal=api-key="$(E2E_API_KEY)" \
		-n default --dry-run=client -o yaml | kubectl apply -f -
	kubectl apply -f tests/e2e/fixtures/sonarr-instance.yaml
	@echo "Waiting for Sonarr to be ready..."
	kubectl wait --for=jsonpath='{.status.conditions[?(@.type=="Ready")].status}'=True \
		sonarr/sonarr -n default --timeout=300s
	@echo "Sonarr instance is ready"

.PHONY: e2e-up
e2e-up: e2e-cluster-create e2e-deploy-sonarr ## Full E2E setup: create cluster + deploy Sonarr
	@echo ""
	@echo "E2E environment is ready."
	@echo "Next steps:"
	@echo "  1. Run the operator:  make run-debug"
	@echo "  2. Run E2E tests:     make e2e"

.PHONY: e2e-down
e2e-down: e2e-cluster-delete ## Tear down E2E environment

.PHONY: e2e
e2e: ## Run E2E tests (requires operator + Sonarr running)
	SONARR_API_KEY=$(E2E_API_KEY) SONARR_URL=$(SONARR_URL) \
		cargo test --test e2e -- --ignored --test-threads=1

.PHONY: e2e-verbose
e2e-verbose: ## Run E2E tests with verbose output
	SONARR_API_KEY=$(E2E_API_KEY) SONARR_URL=$(SONARR_URL) RUST_LOG=debug \
		cargo test --test e2e -- --ignored --test-threads=1 --nocapture

.PHONY: e2e-cleanup
e2e-cleanup: ## Cleanup E2E test resources (without deleting cluster)
	kubectl delete -f tests/e2e/fixtures/sonarr-instance.yaml --ignore-not-found
	kubectl delete secret sonarr-api-key -n default --ignore-not-found
	kubectl delete namespace sonarr-e2e-test --ignore-not-found

##@ Complete Workflows

.PHONY: all
all: lint test build crds ## Run lint, test, build, and generate CRDs

.PHONY: ci
ci: lint test build ## CI pipeline tasks
