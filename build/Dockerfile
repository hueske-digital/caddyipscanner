FROM rust:1-slim AS builder
WORKDIR /app
COPY Cargo.toml .
COPY src ./src
RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add aarch64-unknown-linux-musl && \
    cargo build --release --target aarch64-unknown-linux-musl
FROM scratch
COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/caddyipscanner /app/caddyipscanner
ENTRYPOINT ["/app/caddyipscanner"]