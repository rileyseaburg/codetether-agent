# Build container for codetether-agent
# This container runs the codetether Rust binary which includes:
# - /task CloudEvent endpoint for Knative Eventing
# - A2A protocol server
# - Cognition engine

FROM rust:1.88-slim AS builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    g++ \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Copy source, vendor directory, proto files, and policies
COPY Cargo.toml Cargo.lock build.rs ./
COPY src ./src
COPY vendor ./vendor
COPY proto ./proto
COPY policies ./policies

# Build release binary
RUN cargo build --release --bin codetether

# Final stage - minimal runtime
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /build/target/release/codetether /app/codetether

# Create non-root user
RUN useradd -m -u 1000 codetether && \
    chown -R codetether:codetether /app
USER codetether

# Default port
EXPOSE 8080

ENTRYPOINT ["/app/codetether"]
CMD ["serve", "--hostname", "0.0.0.0", "--port", "8080"]
