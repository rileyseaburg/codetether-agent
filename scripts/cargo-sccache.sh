#!/usr/bin/env bash
set -euo pipefail

if [ -f "$HOME/.bashrc" ]; then
  # shellcheck disable=SC1090
  set +u
  source "$HOME/.bashrc"
  set -u
fi

if command -v sccache >/dev/null 2>&1; then
  export RUSTC_WRAPPER="${RUSTC_WRAPPER:-sccache}"
fi

exec cargo "$@"
