#!/usr/bin/env bash
# Runs only inside the remote Forgejo CUDA build container, never on the workstation.
set -euo pipefail
[[ ${CI:-} == true ]] || { echo 'Forgejo CI execution required' >&2; exit 1; }
mkdir -p bonsai-evidence
export DEBIAN_FRONTEND=noninteractive
bash script/forgejo/apt-https.sh
apt-get update
apt-get install -y --no-install-recommends build-essential pkg-config libssl-dev \
  libasound2-dev protobuf-compiler libprotobuf-dev mold curl ca-certificates git jq
curl --proto '=https' --tlsv1.2 -fsS https://sh.rustup.rs -o /tmp/bonsai-rustup.sh
sh /tmp/bonsai-rustup.sh -y --profile minimal --default-toolchain 1.95.0
export PATH="$HOME/.cargo/bin:/usr/local/cuda/bin:$PATH"
printf '%s\n' "$HOME/.cargo/bin" /usr/local/cuda/bin >> "${GITHUB_PATH:-${FORGEJO_PATH:?}}"
printf '%s\n' CUDA_ROOT=/usr/local/cuda CUDA_PATH=/usr/local/cuda \
  PROTOC=/usr/bin/protoc PROTOC_INCLUDE=/usr/include >> "${GITHUB_ENV:-${FORGEJO_ENV:?}}"
rustc +1.95.0 --version | tee bonsai-evidence/rust-version.txt
nvcc --version | tee bonsai-evidence/cuda-version.txt
git rev-parse HEAD > bonsai-evidence/source-commit.txt