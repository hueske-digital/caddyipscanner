FROM rust:1-slim AS builder
WORKDIR /app
COPY Cargo.toml .
COPY src ./src
RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl && \
    cargo build --release --target $(uname -m)-unknown-linux-musl

FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/caddyipscanner /app/caddyipscanner-x86_64
COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/caddyipscanner /app/caddyipscanner-aarch64
ENTRYPOINT ["sh", "-c", "if [ \"$(uname -m)\" = 'aarch64' ]; then exec /app/caddyipscanner-aarch64; else exec /app/caddyipscanner-x86_64; fi"]