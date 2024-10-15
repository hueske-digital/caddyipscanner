FROM rust:1-slim AS builder
WORKDIR /app
COPY Cargo.toml .
COPY src ./src
RUN apt-get update && apt-get install -y musl-tools
ARG TARGETPLATFORM
RUN if [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
      rustup target add aarch64-unknown-linux-musl && \
      cargo build --release --target aarch64-unknown-linux-musl; \
    elif [ "$TARGETPLATFORM" = "linux/amd64" ]; then \
      rustup target add x86_64-unknown-linux-musl && \
      cargo build --release --target x86_64-unknown-linux-musl; \
    fi

FROM scratch
ARG TARGETPLATFORM
COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/caddyipscanner /app/caddyipscanner-arm64
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/caddyipscanner /app/caddyipscanner-amd64
ENTRYPOINT ["sh", "-c", "if [ \"$TARGETPLATFORM\" = 'linux/arm64' ]; then exec /app/caddyipscanner-arm64; else exec /app/caddyipscanner-amd64; fi"]