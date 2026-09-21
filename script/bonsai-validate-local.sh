#!/usr/bin/env bash
# Run only in a detached, resource-limited user service. Never starts a repair agent.
set -euo pipefail
cd "$(dirname "$0")/.."
[[ ${CODETETHER_BONSAI_BACKGROUND:-} == 1 ]] || {
  echo 'Run this validation through a detached systemd user service.' >&2
  exit 2
}
export PATH="$HOME/.cargo/bin:$PATH"
export CODETETHER_INSTALL_DETACHED=1 CODETETHER_INSTALL_AUTO_FIX=0 CODETETHER_INSTALL_SCCACHE=0
export CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 RUSTC_WRAPPER=
export CARGO_TARGET_DIR="$PWD/target" CARGO_BUILD_BUILD_DIR="$PWD/target"
export CUDA_ROOT=/usr CUDA_PATH=/usr CUDA_HOME=/usr CUDA_COMPUTE_CAP=75
export NVCC_CCBIN=/usr/bin/g++-12 CC=/usr/bin/gcc-12 CXX=/usr/bin/g++-12
export CARGO_PROFILE_RELEASE_LTO=false CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16
bash script/install-dev.sh install --path . --force --locked --features candle-cuda --jobs 1
cargo test --locked --release --features candle-cuda --test bonsai_protocol --jobs 1 -- --test-threads=1
cargo test --locked --release --features candle-cuda --lib provider::bonsai:: --jobs 1 -- --test-threads=1
cargo test --locked --release --features candle-cuda --lib provider::bonsai::native::cuda_ --jobs 1 -- --ignored --test-threads=1 --nocapture
"$HOME/.cargo/bin/codetether" bonsai --prompt 'Reply with exactly BONSAI_NATIVE_OK and nothing else.' --repeat 2
CODETETHER_BIN="$HOME/.cargo/bin/codetether" tetherscript run examples/tetherscript/bonsai_provider.tether
echo 'BONSAI_DIRECT_NATIVE_VALIDATION_COMPLETE'