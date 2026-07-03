#!/usr/bin/env bash
set -uo pipefail

if [ -f "$HOME/.bashrc" ]; then
  # shellcheck disable=SC1090
  set +u
  source "$HOME/.bashrc"
  set -u
fi

if command -v sccache >/dev/null 2>&1; then
  export RUSTC_WRAPPER="${RUSTC_WRAPPER:-sccache}"
fi

# Capture output so we can feed errors to the agent on failure
tmp=$(mktemp)
trap 'rm -f "$tmp"' EXIT

cargo "$@" 2>&1 | tee "$tmp"
exit_code=${PIPESTATUS[0]}

if [ "$exit_code" -ne 0 ] && command -v codetether >/dev/null 2>&1; then
  errors=$(grep -E '^error' "$tmp" | head -80)
  if [ -n "$errors" ]; then
    echo "---"
    echo "Running codetether to fix errors..."
    codetether run "fix these build errors:\n${errors}"
  fi
fi

exit "$exit_code"
