FROM rust:1.88-slim-bookworm AS builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy src to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "" > src/lib.rs && \
    mkdir -p src/bin && echo "fn main() {}" > src/bin/crdgen.rs

# Build dependencies
RUN cargo build --release && rm -rf src

# Copy actual source code
COPY src ./src

# Build the application
RUN touch src/main.rs src/lib.rs && cargo build --release --bin sonarr-operator

# Runtime image - use distroless for smaller image and no apt needed
FROM gcr.io/distroless/cc-debian12:nonroot

COPY --from=builder /app/target/release/sonarr-operator /usr/local/bin/sonarr-operator

ENTRYPOINT ["/usr/local/bin/sonarr-operator"]
