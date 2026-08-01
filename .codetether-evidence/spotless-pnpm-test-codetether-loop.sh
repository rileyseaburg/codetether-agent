#!/usr/bin/env bash
# Fail-fast pnpm validation loop with CodeTether repair between iterations.
# Usage: scripts/pnpm-test-codetether-loop.sh [iteration]
# Example: MAX_ITERATIONS=5 scripts/pnpm-test-codetether-loop.sh
set -uo pipefail
MODEL="${CODETETHER_TEST_MODEL:-gemini-web/gemini-web-fast}"
ACCESS_MODE="${CODETETHER_TEST_ACCESS_MODE:-full}"
MAX_ITERATIONS="${MAX_ITERATIONS:-100}"
REPAIR_MAX_ATTEMPTS="${CODETETHER_REPAIR_MAX_ATTEMPTS:-4}"
REPAIR_MAX_STEPS="${CODETETHER_REPAIR_MAX_STEPS:-12}"
ITERATION="${1:-1}"
REPO_ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
RUN_DIR="$REPO_ROOT/.tmp/codetether-test-loop"
TEST_LOG="$RUN_DIR/pnpm-test-iteration-$ITERATION.log"
AGENT_LOG="$RUN_DIR/codetether-iteration-$ITERATION.log"
LEDGER_DB="$RUN_DIR/passing-tests.sqlite"
SESSION_FILE=""
REPAIR_CONTEXT_FILE=""
LOOP_MODULES=(
  "$REPO_ROOT/scripts/codetether-loop/ledger.sh"
  "$REPO_ROOT/scripts/codetether-loop/target-runner.sh"
  "$REPO_ROOT/scripts/codetether-loop/frontend-runtime.sh"
  "$REPO_ROOT/scripts/codetether-loop/phases.sh"
  "$REPO_ROOT/scripts/codetether-loop/rstest-playwright-phase.sh"
  "$REPO_ROOT/scripts/codetether-loop/metrics.sh"
  "$REPO_ROOT/scripts/codetether-loop/repair-prompt.sh"
  "$REPO_ROOT/scripts/codetether-loop/repair-runner.sh"
)

if ! [[ "$ITERATION" =~ ^[1-9][0-9]*$ && "$MAX_ITERATIONS" =~ ^[1-9][0-9]*$ \
  && "$REPAIR_MAX_ATTEMPTS" =~ ^[1-9][0-9]*$ && "$REPAIR_MAX_STEPS" =~ ^[1-9][0-9]*$ ]]; then
  echo "Iteration and repair limits must be positive integers." >&2
  exit 2
fi

if (( ITERATION > MAX_ITERATIONS )); then
  echo "Stopped after MAX_ITERATIONS=$MAX_ITERATIONS without a passing test run." >&2
  exit 1
fi

for required_command in pnpm codetether sqlite3 jq curl git sha256sum; do
  if ! command -v "$required_command" >/dev/null 2>&1; then
    echo "$required_command is required." >&2
    exit 127
  fi
done

for loop_module in "${LOOP_MODULES[@]}"; do
  if ! bash -n "$loop_module"; then
    echo "Invalid repair-loop module: $loop_module" >&2
    exit 2
  fi
done

mkdir -p "$RUN_DIR"
cd "$REPO_ROOT" || exit 1
: > "$TEST_LOG"

source "$REPO_ROOT/scripts/codetether-loop/ledger.sh"
initialize_test_ledger
if [[ "$ITERATION" == "1" && "${RESET_TEST_LEDGER:-0}" == "1" ]]; then
  sqlite3 "$LEDGER_DB" 'DELETE FROM signed_passes;'
  echo "Cleared signed passes; attempt history was preserved."
fi
source "$REPO_ROOT/scripts/codetether-loop/target-runner.sh"
source "$REPO_ROOT/scripts/codetether-loop/frontend-runtime.sh"
source "$REPO_ROOT/scripts/codetether-loop/phases.sh"
source "$REPO_ROOT/scripts/codetether-loop/rstest-playwright-phase.sh"
source "$REPO_ROOT/scripts/codetether-loop/metrics.sh"
source "$REPO_ROOT/scripts/codetether-loop/repair-prompt.sh"
source "$REPO_ROOT/scripts/codetether-loop/repair-runner.sh"

PNPM_STATUS=0
run_all_phases || PNPM_STATUS="$?"
declare -F record_and_commit_test_metrics >/dev/null || source "$REPO_ROOT/scripts/codetether-loop/metrics.sh"
record_and_commit_test_metrics "$PNPM_STATUS" || exit 1

if (( PNPM_STATUS == 0 )); then
  echo "All test targets passed or were already recorded on iteration $ITERATION."
  exit 0
fi

echo "Stopped at first failing target \"$FAILED_TARGET\" with exit $PNPM_STATUS." >&2
echo "Starting CodeTether repair for that target..."
REPAIR_KEY="$(printf '%s' "$MODEL:$FAILED_TARGET" | sha256sum)"
REPAIR_KEY="${REPAIR_KEY%% *}"
AGENT_LOG="$RUN_DIR/codetether-${REPAIR_KEY:0:16}.log"
SESSION_FILE="$RUN_DIR/codetether-session-${REPAIR_KEY:0:16}.id"
REPAIR_CONTEXT_FILE="$RUN_DIR/repair-context-${REPAIR_KEY:0:16}.json"
REPAIR_BRANCH="$(git branch --show-current)"
REPAIR_HEAD="$(git rev-parse HEAD)"
if [[ -z "$REPAIR_BRANCH" ]]; then
  echo "CodeTether repair requires a checked-out branch." >&2
  exit 1
fi
if ! repair_until_commit; then
  exit 1
fi

echo "CodeTether committed its repair on $REPAIR_BRANCH. Recursing into iteration $((ITERATION + 1))..."
exec bash "$REPO_ROOT/scripts/pnpm-test-codetether-loop.sh" "$((ITERATION + 1))"