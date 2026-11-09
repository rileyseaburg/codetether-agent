#!/usr/bin/env bash
# Compile on Forgejo; export binaries so the workstation never needs Cargo.
set -euo pipefail
[[ ${CI:-} == true ]] || { echo 'Forgejo CI execution required' >&2; exit 1; }
export PATH="$HOME/.cargo/bin:/usr/local/cuda-12.1/bin:$PATH"
export CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=/tmp/bonsai-target CARGO_INCREMENTAL=0
export CARGO_PROFILE_CI_DEBUG=0 CARGO_PROFILE_CI_CODEGEN_UNITS=256
export CUDA_ROOT=/usr/local/cuda-12.1 CUDA_PATH=/usr/local/cuda-12.1 CUDA_HOME=/usr/local/cuda-12.1 CUDA_COMPUTE_CAP=75
export LD_LIBRARY_PATH="/usr/local/cuda-12.1/lib64${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
mkdir -p bonsai-evidence bonsai-dist
report_failure() {
  local code=$?
  if (( code != 0 )) && [[ -f bonsai-evidence/test-compile.jsonl ]]; then
    jq -r 'select(.reason=="compiler-message")|.message.rendered // empty' bonsai-evidence/test-compile.jsonl | tail -100
  fi
  return "$code"
}
trap report_failure EXIT
cargo +1.95.0 test --locked --profile ci --features candle-cuda --lib --no-run \
  --message-format=json > bonsai-evidence/test-compile.jsonl \
  2> >(tee bonsai-evidence/test-compile.stderr >&2)
test_binary=$(jq -r 'select(.reason=="compiler-artifact" and .profile.test==true and .target.name=="codetether_agent")|.executable // empty' bonsai-evidence/test-compile.jsonl | tail -1)
test -x "$test_binary"
# CPU reference tests never initialize CUDA, but the ELF loader needs libcuda's SONAME.
# Do not package this link: GPU validation must use the workstation's real driver.
mkdir -p /tmp/bonsai-cuda-stubs
ln -sf /usr/local/cuda-12.1/lib64/stubs/libcuda.so /tmp/bonsai-cuda-stubs/libcuda.so.1
export LD_LIBRARY_PATH="/tmp/bonsai-cuda-stubs:$LD_LIBRARY_PATH"
"$test_binary" provider::bonsai::native:: --test-threads=1 \
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
