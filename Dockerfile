FROM rust:1.85 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/k8s-operator-sonarr /usr/local/bin/

ENTRYPOINT ["k8s-operator-sonarr"]
