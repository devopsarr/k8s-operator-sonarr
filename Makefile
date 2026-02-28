# Sonarr Kubernetes Operator Makefile
#
# Usage:
#   make help      - Show this help
#   make check     - Run cargo check
#   make build     - Build release binary
#   make crds      - Generate CRD manifests
#   make install   - Install CRDs to cluster
#   make run       - Run operator locally
#   make docker    - Build Docker image
#   make test      - Run tests
#   make e2e       - Run end-to-end tests

# Configuration
BINARY_NAME := sonarr-operator
IMAGE_NAME := sonarr-operator
IMAGE_TAG := latest
NAMESPACE := sonarr-operator-system
CRD_DIR := deploy/crds

# Kubernetes context (optional, uses current context if not set)
# KUBECONFIG := ~/.kube/config

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

.PHONY: test
test: ## Run unit tests
	cargo test

.PHONY: test-verbose
test-verbose: ## Run tests with verbose output
	cargo test -- --nocapture

.PHONY: integration-test
integration-test: ## Run integration tests (requires cluster and CRDs installed)
	cargo test --test integration -- --ignored --test-threads=1

.PHONY: integration-test-verbose
integration-test-verbose: ## Run integration tests with verbose output
	cargo test --test integration -- --ignored --test-threads=1 --nocapture

.PHONY: integration-setup
integration-setup: kind-create install ## Setup integration test environment
	@echo "Integration test environment ready"
	@echo "Run 'make integration-test' to execute tests"

.PHONY: integration-cleanup
integration-cleanup: ## Cleanup integration test resources
	kubectl delete namespace sonarr-operator-test --ignore-not-found
	@echo "Integration test resources cleaned up"

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
	rm -rf $(CRD_DIR)

##@ CRD Management

.PHONY: crds
crds: ## Generate CRD manifests (individual files)
	@mkdir -p $(CRD_DIR)
	@rm -f $(CRD_DIR)/*.yaml
	cargo run --bin crdgen -- --split $(CRD_DIR)

.PHONY: crds-combined
crds-combined: ## Generate all CRDs in a single file
	@mkdir -p $(CRD_DIR)
	cargo run --bin crdgen > $(CRD_DIR)/crds.yaml
	@echo "CRDs generated in $(CRD_DIR)/crds.yaml"

.PHONY: crds-single
crds-single: ## Generate a single CRD (use: make crds-single CRD=SonarrTag)
	@mkdir -p $(CRD_DIR)
	cargo run --bin crdgen -- --single $(CRD) > $(CRD_DIR)/$(shell echo $(CRD) | tr '[:upper:]' '[:lower:]').yaml

##@ Documentation

DOCS_DIR := docs/api
GOBIN := $(shell go env GOPATH)/bin

.PHONY: docs
docs: crds ## Generate CRD documentation
	@mkdir -p $(DOCS_DIR)
	@echo "Generating CRD documentation..."
	@if ! command -v crdoc >/dev/null 2>&1 && [ ! -f "$(GOBIN)/crdoc" ]; then \
		echo "Installing crdoc..."; \
		go install fybrik.io/crdoc@latest; \
	fi
	@PATH="$(GOBIN):$$PATH" crdoc --resources $(CRD_DIR) --output $(DOCS_DIR)/crd-reference.md
	@echo "Documentation generated in $(DOCS_DIR)/crd-reference.md"

.PHONY: docs-html
docs-html: docs ## Generate HTML documentation (requires pandoc)
	@command -v pandoc >/dev/null 2>&1 || { echo "ERROR: pandoc not installed"; exit 1; }
	pandoc $(DOCS_DIR)/crd-reference.md -o $(DOCS_DIR)/crd-reference.html --standalone --toc --metadata title="Sonarr Operator CRD Reference"
	@echo "HTML documentation generated in $(DOCS_DIR)/crd-reference.html"

.PHONY: install
install: crds ## Install CRDs to cluster
	kubectl apply -f $(CRD_DIR)/

.PHONY: uninstall
uninstall: ## Remove CRDs from cluster
	kubectl delete -f $(CRD_DIR)/ --ignore-not-found

##@ Running

.PHONY: run
run: ## Run operator locally (requires kubeconfig)
	RUST_LOG=info cargo run --bin sonarr-operator

.PHONY: run-debug
run-debug: ## Run operator with debug logging
	RUST_LOG=debug cargo run --bin sonarr-operator

##@ Docker

.PHONY: docker
docker: ## Build Docker image
	docker build -t $(IMAGE_NAME):$(IMAGE_TAG) .

.PHONY: docker-push
docker-push: docker ## Push Docker image to registry
	docker push $(IMAGE_NAME):$(IMAGE_TAG)

##@ Kubernetes Deployment

.PHONY: deploy
deploy: install ## Deploy operator to cluster
	kubectl apply -f deploy/namespace.yaml
	kubectl apply -f deploy/rbac.yaml
	kubectl apply -f deploy/deployment.yaml

.PHONY: undeploy
undeploy: ## Remove operator from cluster
	kubectl delete -f deploy/deployment.yaml --ignore-not-found
	kubectl delete -f deploy/rbac.yaml --ignore-not-found
	kubectl delete -f deploy/namespace.yaml --ignore-not-found

##@ Testing with Kind

.PHONY: kind-create
kind-create: ## Create a Kind cluster for testing
	kind create cluster --name sonarr-operator-test

.PHONY: kind-delete
kind-delete: ## Delete the Kind cluster
	kind delete cluster --name sonarr-operator-test

.PHONY: kind-load
kind-load: docker ## Load Docker image into Kind cluster
	kind load docker-image $(IMAGE_NAME):$(IMAGE_TAG) --name sonarr-operator-test

##@ End-to-End Testing
#
# The E2E flow is:
#   1. Create a k3d/Kind cluster and install CRDs
#   2. Run the operator (locally or in-cluster)
#   3. Create an API key Secret + apply the Sonarr CR
#   4. The operator creates the Sonarr Deployment, Service, PVC
#   5. Run E2E tests against the live Sonarr instance
#
# Required env vars: SONARR_API_KEY, SONARR_URL

E2E_API_KEY ?= test-e2e-api-key-12345

.PHONY: e2e-deploy-sonarr
e2e-deploy-sonarr: install ## Create API key secret + apply Sonarr CR (operator must be running)
	@echo "Creating API key secret..."
	kubectl create secret generic sonarr-api-key \
		--from-literal=api-key="$(E2E_API_KEY)" \
		-n default --dry-run=client -o yaml | kubectl apply -f -
	@echo "Applying Sonarr CR..."
	kubectl apply -f tests/e2e/fixtures/sonarr-instance.yaml
	@echo "Waiting for Sonarr to be ready..."
	kubectl wait --for=jsonpath='{.status.conditions[?(@.type=="Ready")].status}'=True \
		sonarr/sonarr -n default --timeout=300s
	@echo "Sonarr instance is ready"

.PHONY: e2e
e2e: ## Run E2E tests (requires operator + Sonarr running)
	@echo "Running E2E tests..."
	@if [ -z "$$SONARR_API_KEY" ]; then \
		echo "ERROR: SONARR_API_KEY environment variable not set"; \
		echo "Run: export SONARR_API_KEY=$(E2E_API_KEY)"; \
		exit 1; \
	fi
	@if [ -z "$$SONARR_URL" ]; then \
		export SONARR_URL="http://localhost:8989"; \
	fi
	cargo test --test e2e -- --ignored --test-threads=1

.PHONY: e2e-verbose
e2e-verbose: ## Run E2E tests with verbose output
	@echo "Running E2E tests (verbose)..."
	@if [ -z "$$SONARR_API_KEY" ]; then \
		echo "ERROR: SONARR_API_KEY environment variable not set"; \
		exit 1; \
	fi
	RUST_LOG=debug cargo test --test e2e -- --ignored --test-threads=1 --nocapture

.PHONY: e2e-cleanup
e2e-cleanup: ## Cleanup E2E test resources
	kubectl delete -f tests/e2e/fixtures/sonarr-instance.yaml --ignore-not-found
	kubectl delete secret sonarr-api-key -n default --ignore-not-found
	kubectl delete namespace sonarr-e2e-test --ignore-not-found
	@echo "E2E resources cleaned up"

##@ Complete Workflows

.PHONY: all
all: lint test build crds ## Run lint, test, build, and generate CRDs

.PHONY: ci
ci: lint test build ## CI pipeline tasks
