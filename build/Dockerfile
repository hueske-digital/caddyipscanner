# First stage: Build for the respective platform
FROM rust:1-slim AS builder

WORKDIR /app
COPY Cargo.toml .
COPY src ./src

# Install musl-tools
RUN apt-get update && apt-get install -y musl-tools

# Build for the appropriate target based on the platform
ARG TARGETPLATFORM
RUN if [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
      rustup target add aarch64-unknown-linux-musl && \
      cargo build --release --target aarch64-unknown-linux-musl; \
    elif [ "$TARGETPLATFORM" = "linux/amd64" ]; then \
      rustup target add x86_64-unknown-linux-musl && \
      cargo build --release --target x86_64-unknown-linux-musl; \
    fi

# Second stage: Copy the correct binary
FROM scratch

ARG TARGETPLATFORM
# Copy the correct binary depending on the platform
RUN if [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
      cp /app/target/aarch64-unknown-linux-musl/release/caddyipscanner /app/caddyipscanner; \
    elif [ "$TARGETPLATFORM" = "linux/amd64" ]; then \
      cp /app/target/x86_64-unknown-linux-musl/release/caddyipscanner /app/caddyipscanner; \
    fi

# Set the entrypoint
ENTRYPOINT ["/app/caddyipscanner"]