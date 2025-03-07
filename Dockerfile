FROM rust:1.71 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust-k8s-operator /usr/local/bin/

ENTRYPOINT ["k8s-opeartor-sonarr"]
