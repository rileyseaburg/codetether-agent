# syntax=docker/dockerfile:1.7
FROM rust:1.95-slim-bookworm AS builder

WORKDIR /build

COPY docker/release/apt-https.sh /usr/local/share/codetether/apt-https.sh

RUN sh /usr/local/share/codetether/apt-https.sh && apt-get update && apt-get install -y \
    mingw-w64 \
    pkg-config \
    libssl-dev \
    g++ \
    protobuf-compiler \
    libprotobuf-dev \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-pc-windows-gnu

RUN cargo install sccache --locked
ARG RUSTC_WRAPPER=sccache
ARG SCCACHE_GHA_ENABLED=false
ENV RUSTC_WRAPPER=${RUSTC_WRAPPER}
ENV SCCACHE_GHA_ENABLED=${SCCACHE_GHA_ENABLED}
ENV CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc

COPY Cargo.toml Cargo.lock build.rs ./
COPY src ./src
COPY crates ./crates
COPY vendor ./vendor
COPY proto ./proto
COPY policies ./policies
COPY examples ./examples

# Host proc-macros must not come from another builder's incompatible libc.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,id=codetether-windows-bookworm-rust195,target=/build/target,sharing=locked \
    cargo build --locked --release --target x86_64-pc-windows-gnu --bin codetether && \
    mkdir -p /artifacts && \
    cp /build/target/x86_64-pc-windows-gnu/release/codetether.exe /artifacts/codetether.exe

FROM scratch AS artifact

COPY --from=builder /artifacts/codetether.exe /codetether.exe