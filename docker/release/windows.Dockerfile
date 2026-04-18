FROM rust:1.88-slim AS builder

WORKDIR /build

RUN apt-get update && apt-get install -y \
    mingw-w64 \
    pkg-config \
    libssl-dev \
    g++ \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-pc-windows-gnu

ENV CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc

COPY Cargo.toml Cargo.lock build.rs ./
COPY src ./src
COPY vendor ./vendor
COPY proto ./proto
COPY policies ./policies

RUN cargo build --release --target x86_64-pc-windows-gnu --bin codetether --features functiongemma

FROM scratch AS artifact

COPY --from=builder /build/target/x86_64-pc-windows-gnu/release/codetether.exe /codetether.exe
