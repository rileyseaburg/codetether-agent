#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../.." && pwd)"
TMP_DIR="$(mktemp -d "$ROOT/.tmp/codetether-large-context-test.XXXXXX")"
trap 'rm -rf -- "$TMP_DIR"' EXIT

FAILED_TARGET='playwright:native'
ITERATION=1
TEST_LOG="$TMP_DIR/test.log"
REPO_ROOT="$ROOT"
REPAIR_CONTEXT_FILE="$TMP_DIR/context.json"
MODEL='gemini-web/gemini-web-fast'
ACCESS_MODE=full
REPAIR_MAX_STEPS=12
AGENT_LOG="$TMP_DIR/agent.log"
SESSION_FILE="$TMP_DIR/session.id"
export FAILED_TARGET ITERATION TEST_LOG REPO_ROOT REPAIR_CONTEXT_FILE
export MODEL ACCESS_MODE REPAIR_MAX_STEPS AGENT_LOG SESSION_FILE

awk -v marker="Running target: $FAILED_TARGET" 'BEGIN {
  print marker
  for (i = 1; i <= 6000; i++) printf "failure line %05d: %080d\n", i, i
}' > "$TEST_LOG"

source "$ROOT/scripts/codetether-loop/repair-prompt.sh"
source "$ROOT/scripts/codetether-loop/repair-runner.sh"
persist_repair_session() { return 0; }
codetether() {
  local argument
  for argument in "$@"; do
    ((${#argument} < 131072)) || return 126
  done
  [[ "$2" == *"$REPAIR_CONTEXT_FILE"* ]]
  [[ " $* " == *" --file $REPAIR_CONTEXT_FILE "* ]]
  printf '%s\n' '{"text":"ok","session_id":"11111111-1111-1111-1111-111111111111"}'
}

run_repair_turn >/dev/null
prompt="$(build_repair_prompt)"
(( ${#prompt} < 10000 ))
jq -e '(.failure_output_lines | join("\n") | length) > 400000' \
  "$REPAIR_CONTEXT_FILE" >/dev/null
echo "large repair context regression: passed"