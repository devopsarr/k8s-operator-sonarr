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

.PHONY: e2e-setup
e2e-setup: kind-create install ## Setup E2E test environment with Sonarr
	@echo "Setting up E2E test environment..."
	chmod +x tests/e2e/fixtures/setup.sh
	./tests/e2e/fixtures/setup.sh
	@echo "E2E environment ready"

.PHONY: e2e-sonarr
e2e-sonarr: ## Deploy only Sonarr for E2E tests (use after kind-create install)
	@echo "Deploying Sonarr..."
	kubectl apply -f tests/e2e/fixtures/sonarr-deployment.yaml
	kubectl wait --for=condition=available deployment/sonarr -n default --timeout=300s
	kubectl wait --for=condition=ready pod -l app=sonarr -n default --timeout=300s
	@echo "Waiting for Sonarr to initialize..."
	sleep 60
	@echo "Extracting API key..."
	@SONARR_POD=$$(kubectl get pod -l app=sonarr -n default -o jsonpath='{.items[0].metadata.name}'); \
	API_KEY=$$(kubectl exec $$SONARR_POD -n default -- cat /config/config.xml 2>/dev/null | grep -oP '(?<=<ApiKey>)[^<]+'); \
	echo "API Key: $$API_KEY"; \
	kubectl create secret generic sonarr-api-key --from-literal=api-key="$$API_KEY" -n default --dry-run=client -o yaml | kubectl apply -f -
	kubectl apply -f tests/e2e/fixtures/sonarr-instance.yaml
	@echo "Sonarr deployed and configured"

.PHONY: e2e
e2e: ## Run E2E tests (requires e2e-setup or e2e-sonarr first, operator running)
	@echo "Running E2E tests..."
	@if [ -z "$$SONARR_API_KEY" ]; then \
		echo "ERROR: SONARR_API_KEY environment variable not set"; \
		echo "Run: export SONARR_API_KEY=\$$(cat tests/e2e/fixtures/.sonarr-api-key)"; \
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

.PHONY: e2e-port-forward
e2e-port-forward: ## Start port-forward to Sonarr (run in separate terminal)
	@echo "Port-forwarding to Sonarr on localhost:8989..."
	@echo "Press Ctrl+C to stop"
	kubectl port-forward svc/sonarr 8989:8989 -n default

.PHONY: e2e-cleanup
e2e-cleanup: ## Cleanup E2E test resources
	chmod +x tests/e2e/fixtures/cleanup.sh
	./tests/e2e/fixtures/cleanup.sh

.PHONY: e2e-full
e2e-full: e2e-setup ## Run full E2E test suite (setup + tests + cleanup)
	@echo "Starting operator in background..."
	RUST_LOG=info cargo run &
	sleep 10
	@echo "Setting up port-forward..."
	kubectl port-forward svc/sonarr 8989:8989 -n default &
	sleep 5
	@echo "Running E2E tests..."
	export SONARR_API_KEY=$$(cat tests/e2e/fixtures/.sonarr-api-key); \
	export SONARR_URL="http://localhost:8989"; \
	cargo test --test e2e -- --ignored --test-threads=1 || true
	@echo "Cleaning up..."
	pkill -f "cargo run" || true
	pkill -f "port-forward" || true
	$(MAKE) e2e-cleanup

##@ Complete Workflows

.PHONY: all
all: lint test build crds ## Run lint, test, build, and generate CRDs

.PHONY: ci
ci: lint test build ## CI pipeline tasks
