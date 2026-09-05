#!/usr/bin/env bash
# CI runner prerequisites only; callers select the target explicitly.
set -euo pipefail
if [ "${CI:-}" != true ]; then echo 'This setup is CI-only' >&2; exit 1; fi
target=${1:?Rust target is required}
case "$(uname -s)" in
  Linux)
    elevate=()
    if [ "$(id -u)" -ne 0 ]; then elevate=(sudo); fi
    "${elevate[@]}" sh "$(dirname "$0")/apt-https.sh"
    "${elevate[@]}" apt-get update
    "${elevate[@]}" apt-get install -y --no-install-recommends \
      build-essential pkg-config libssl-dev libasound2-dev protobuf-compiler libprotobuf-dev \
      mold curl ca-certificates git jq
    test -f /usr/include/google/protobuf/struct.proto
    printf '%s\n' PROTOC=/usr/bin/protoc PROTOC_INCLUDE=/usr/include \
      >> "${GITHUB_ENV:-${FORGEJO_ENV:?}}"
    ;;
  Darwin)
    export PATH="/opt/homebrew/bin:/usr/local/bin:$PATH"
    brew install protobuf
    printf 'PROTOC=%s/bin/protoc\nPROTOC_INCLUDE=%s/include\n' "$(brew --prefix protobuf)" "$(brew --prefix protobuf)" \
      >> "${GITHUB_ENV:-${FORGEJO_ENV:?}}"
    ;;
  *) echo 'Unsupported Unix runner' >&2; exit 1 ;;
esac
if ! command -v rustup >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- -y --profile minimal --default-toolchain none
fi
export PATH="$HOME/.cargo/bin:$PATH"
echo "$HOME/.cargo/bin" >> "${GITHUB_PATH:-${FORGEJO_PATH:?}}"
if [ "$(uname -s)" = Darwin ]; then
  printf '%s\n' /opt/homebrew/bin /usr/local/bin >> "${GITHUB_PATH:-${FORGEJO_PATH:?}}"
fi
rustup toolchain install 1.95.0 --profile minimal --component rustfmt --component clippy
rustup target add --toolchain 1.95.0 "$target"