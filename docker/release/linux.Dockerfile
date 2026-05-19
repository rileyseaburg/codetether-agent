FROM rust:1.89-slim AS builder

WORKDIR /build

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libasound2-dev \
    g++ \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock build.rs ./
COPY src ./src
COPY crates ./crates
COPY vendor ./vendor
COPY proto ./proto
COPY policies ./policies
COPY examples ./examples

RUN cargo build --release --bin codetether

FROM scratch AS artifact

COPY --from=builder /build/target/release/codetether /codetether
