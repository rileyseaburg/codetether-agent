#!/usr/bin/env bash
set -uo pipefail

cd "$(dirname "$0")/.." || exit 1

if [ -f "$HOME/.bashrc" ]; then
  # shellcheck disable=SC1090
  set +u
  source "$HOME/.bashrc"
  set -u
fi

if command -v sccache >/dev/null 2>&1; then
  export RUSTC_WRAPPER="${RUSTC_WRAPPER:-sccache}"
fi

# Reuse intermediate artifacts from previous builds
export CARGO_BUILD_BUILD_DIR="$(pwd)/target"

# Capture output so we can feed errors to the agent on failure
tmp=$(mktemp)
trap 'rm -f "$tmp"' EXIT

# Default to install --path . if no args given
args=("$@")
if [ ${#args[@]} -eq 0 ]; then
  args=(install --path .)
fi

while true; do
  cargo "${args[@]}" 2>&1 | tee "$tmp"
  exit_code=${PIPESTATUS[0]}

  if [ "$exit_code" -eq 0 ]; then
    echo "---"
    echo "Build succeeded."
    exit 0
  fi

  if ! command -v codetether >/dev/null 2>&1; then
    exit "$exit_code"
  fi

  errors=$(grep -E '^error' "$tmp" | head -80)
  if [ -z "$errors" ]; then
    exit "$exit_code"
  fi

  echo "---"
  echo "Build failed. Running codetether to fix errors..."
  codetether run --model zai/glm-5.1 --access-mode full "fix these build errors:

${errors}"

  echo "---"
  echo "Retrying build..."
done
