FROM rust:1.88-slim AS builder

WORKDIR /build

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    g++ \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock build.rs ./
COPY src ./src
COPY vendor ./vendor
COPY proto ./proto
COPY policies ./policies

RUN cargo build --release --bin codetether --features functiongemma

FROM scratch AS artifact

COPY --from=builder /build/target/release/codetether /codetether
