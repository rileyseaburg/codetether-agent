#!/usr/bin/env bash
# Self-healing Windows (Docker) build loop.
#
# Runs `make build-windows-docker`, and on failure feeds the build errors to
# codetether to auto-fix, then retries. On success, commits the changes via
# commit.sh.
#
# Usage:
#   ./script/build-windows-dev.sh                # build, heal, commit + push
#   ./script/build-windows-dev.sh --no-push      # commit without pushing
#   ./script/build-windows-dev.sh --no-commit    # build + heal only
#   MAX_HEAL_ITERS=10 ./script/build-windows-dev.sh
set -uo pipefail

cd "$(dirname "$0")/.." || exit 1

if [ -f "$HOME/.bashrc" ]; then
  # shellcheck disable=SC1090
  set +u
  source "$HOME/.bashrc"
  set -u
fi

COMMIT=true
PUSH_ARG=""
for arg in "$@"; do
  case "$arg" in
    --no-commit) COMMIT=false ;;
    --no-push) PUSH_ARG="--no-push" ;;
  esac
done

MAX_HEAL_ITERS="${MAX_HEAL_ITERS:-8}"
MODEL="minimax/m3"

tmp=$(mktemp)
trap 'rm -f "$tmp"' EXIT

iter=0
while true; do
  make build-windows-docker 2>&1 | tee "$tmp"
  exit_code=${PIPESTATUS[0]}

  if [ "$exit_code" -eq 0 ]; then
    echo "---"
    echo "Windows build succeeded."
    break
  fi

  if ! command -v codetether >/dev/null 2>&1; then
    echo "codetether not found on PATH; cannot self-heal." >&2
    exit "$exit_code"
  fi

  iter=$((iter + 1))
  if [ "$iter" -gt "$MAX_HEAL_ITERS" ]; then
    echo "Reached MAX_HEAL_ITERS=$MAX_HEAL_ITERS without a green build." >&2
    exit "$exit_code"
  fi

  errors=$(grep -E 'error(\[|:| )' "$tmp" | head -80)
  if [ -z "$errors" ]; then
    echo "Build failed with no recognizable 'error' lines; not self-healing." >&2
    exit "$exit_code"
  fi

  echo "---"
  echo "Windows build failed (attempt $iter/$MAX_HEAL_ITERS). Running codetether to fix..."
  cd ../../ && codetether run "fix these Windows build errors from 'make build-windows-docker': 
  ${errors}" --model openai-codex/gpt-5.5-fast --access-mode full 

  echo "---"
  echo "Retrying Windows build..."
done

if [ "$COMMIT" = "true" ]; then
  echo "---"
  echo "Committing changes..."
  ./commit.sh $PUSH_ARG
fi
