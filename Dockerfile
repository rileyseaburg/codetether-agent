# Build container for codetether-agent
# This container runs the codetether Rust binary which includes:
# - /task CloudEvent endpoint for Knative Eventing
# - A2A protocol server
# - Cognition engine

FROM rust:1.95-slim-bookworm AS builder

WORKDIR /build

COPY docker/release/apt-https.sh /usr/local/share/codetether/apt-https.sh

# Install build dependencies
RUN sh /usr/local/share/codetether/apt-https.sh && apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libasound2-dev \
    g++ \
    protobuf-compiler \
    libprotobuf-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy source, vendor directory, proto files, policies, and examples
COPY Cargo.toml Cargo.lock build.rs ./
COPY src ./src
COPY crates ./crates
COPY vendor ./vendor
COPY proto ./proto
COPY policies ./policies
COPY examples ./examples

# Build release binary
RUN cargo build --locked --release --bin codetether

# Final stage - minimal runtime
FROM debian:bookworm-slim

# Bootstrap HTTPS trust before the runtime installs its own CA package.
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY docker/release/apt-https.sh /usr/local/share/codetether/apt-https.sh

# Install runtime dependencies
RUN sh /usr/local/share/codetether/apt-https.sh && apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libasound2 \
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