# syntax=docker/dockerfile:1.7
FROM rust:1.89-slim AS builder

WORKDIR /build

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libasound2-dev \
    g++ \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install sccache --locked
ARG RUSTC_WRAPPER=sccache
ARG SCCACHE_GHA_ENABLED=false
ENV RUSTC_WRAPPER=${RUSTC_WRAPPER}
ENV SCCACHE_GHA_ENABLED=${SCCACHE_GHA_ENABLED}
COPY Cargo.toml Cargo.lock build.rs ./
COPY src ./src
COPY crates ./crates
COPY vendor ./vendor
COPY proto ./proto
COPY policies ./policies
COPY examples ./examples

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/build/target \
    cargo build --release --bin codetether && \
    mkdir -p /out && \
    cp /build/target/release/codetether /out/codetether

FROM scratch AS artifact

COPY --from=builder /out/codetether /codetether
