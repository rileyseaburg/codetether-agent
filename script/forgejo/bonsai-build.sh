#!/usr/bin/env bash
# Compile on Forgejo; export binaries so the workstation never needs Cargo.
set -euo pipefail
[[ ${CI:-} == true ]] || { echo 'Forgejo CI execution required' >&2; exit 1; }
export PATH="$HOME/.cargo/bin:/usr/local/cuda/bin:$PATH"
export CUDA_ROOT=/usr/local/cuda CUDA_PATH=/usr/local/cuda CUDA_COMPUTE_CAP=75
export LD_LIBRARY_PATH="/usr/local/cuda/lib64${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
mkdir -p bonsai-evidence bonsai-dist
cargo +1.95.0 test --locked --profile ci --features candle-cuda --lib --no-run \
  --message-format=json > bonsai-evidence/test-compile.jsonl 2> bonsai-evidence/test-compile.stderr
test_binary=$(jq -r 'select(.reason=="compiler-artifact" and .profile.test==true and .target.name=="codetether_agent")|.executable // empty' bonsai-evidence/test-compile.jsonl | tail -1)
test -x "$test_binary"
# CPU reference tests never initialize CUDA, but the ELF loader needs libcuda's SONAME.
# Do not package this link: GPU validation must use the workstation's real driver.
mkdir -p /tmp/bonsai-cuda-stubs
ln -sf /usr/local/cuda/lib64/stubs/libcuda.so /tmp/bonsai-cuda-stubs/libcuda.so.1
export LD_LIBRARY_PATH="/tmp/bonsai-cuda-stubs:$LD_LIBRARY_PATH"
"$test_binary" cognition::thinker::candle::bonsai:: --test-threads=1 \
  2>&1 | tee bonsai-evidence/cpu-reference-tests.txt
cargo +1.95.0 build --locked --profile ci --features candle-cuda --bin codetether \
  2>&1 | tee bonsai-evidence/binary-build.txt
cp "$test_binary" bonsai-dist/bonsai-native-tests
cp "$CARGO_TARGET_DIR/ci/codetether" bonsai-dist/codetether
cp bonsai-evidence/source-commit.txt bonsai-dist/
printf '%s\n' 'features=candle-cuda' 'cuda_compute_cap=75' > bonsai-dist/build-info.txt
(cd bonsai-dist && sha256sum codetether bonsai-native-tests > SHA256SUMS)
tar -czf bonsai-dist/native-cuda75.tar.gz -C bonsai-dist codetether bonsai-native-tests source-commit.txt build-info.txt SHA256SUMS
# Keep uncompressed build outputs in the job filesystem; upload only the archive.
mkdir -p bonsai-evidence/binaries
mv bonsai-dist/codetether bonsai-dist/bonsai-native-tests bonsai-evidence/binaries/
