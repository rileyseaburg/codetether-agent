#!/usr/bin/env bash
# Fail-fast validation phases for the pnpm repair loop.
# Source after target-runner.sh with REPO_ROOT, RUN_DIR, TEST_LOG, ITERATION set.
# Example: source scripts/codetether-loop/phases.sh; run_all_phases || echo failed
set -uo pipefail

# Runs each typecheck target and returns on the first failing one.
# Example: run_typecheck_phase || echo "typecheck failed on $FAILED_TARGET"
run_typecheck_phase() {
  local target
  for target in typecheck:ui typecheck:api typecheck:rstest typecheck:playwright; do
    run_target_or_stop "$target" pnpm run "$target" || return "$?"
  done
}

# Runs every Rstest spec individually, stopping at the first failing spec file.
# Example: run_rstest_phase || echo "rstest failed on $FAILED_TARGET"
run_rstest_phase() {
  local list="$RUN_DIR/rstest-list-$ITERATION.txt" test_path relative_path
  pnpm exec rstest list --filesOnly >"$list" 2>>"$TEST_LOG" || {
    FAILED_TARGET="rstest list"
    echo "Stopping: rstest list failed." | tee -a "$TEST_LOG"
    return 1
  }
  while IFS= read -r test_path; do
    relative_path="${test_path#"$REPO_ROOT"/}"
    run_target_or_stop "rstest:$relative_path" pnpm exec rstest run \
      "$test_path" || return "$?"
  done <"$list"
}

# Returns success when a Playwright spec directly calls an HTTP API.
# The path convention covers API-owned suites; the source scan also recognizes
# Playwright request fixtures used by UI-domain integration specs.
# Example: is_api_integration_spec tests/playwright/admin/api-keys.spec.ts
is_api_integration_spec() {
  local test_path="$1"
  [[ "$test_path" == "$REPO_ROOT/tests/playwright/api/"* ]] ||
    rg -q '\b(request|page\.request)\.(get|post|put|patch|delete|fetch)\(' "$test_path"
}

# Resolves paths emitted relative to Playwright's configured test directory.
# Example: playwright_source_path admin/api-keys.spec.ts
playwright_source_path() {
  local test_path="$1"
  case "$test_path" in
    /*) printf '%s\n' "$test_path" ;;
    tests/playwright/*) printf '%s/%s\n' "$REPO_ROOT" "$test_path" ;;
    *) printf '%s/tests/playwright/%s\n' "$REPO_ROOT" "$test_path" ;;
  esac
}

# Waits through expected API/frontend rebuilds before running a command.
# Arguments are the command and its arguments.
# Example: run_after_frontend_ready pnpm run test:playwright:all
run_after_frontend_ready() {
  ensure_frontend_stack || return "$?"
  "$@"
}

# Runs every Playwright spec individually, stopping at the first failing spec.
# Example: run_playwright_phase || echo "playwright failed on $FAILED_TARGET"
run_playwright_phase() {
  local files="$RUN_DIR/playwright-files-$ITERATION.txt" test_path relative_path source_path target
  local -a command
  bash "$REPO_ROOT/scripts/codetether-loop/playwright-test-files.sh" \
    "$RUN_DIR/playwright-list-$ITERATION.json" "$TEST_LOG" >"$files" || {
    FAILED_TARGET="playwright --list"
    echo "Stopping: playwright --list failed." | tee -a "$TEST_LOG"
    return 1
  }
  ensure_frontend_stack || {
    FAILED_TARGET="playwright frontend preflight"
    return 1
  }
  while IFS= read -r test_path; do
    source_path="$(playwright_source_path "$test_path")"
    relative_path="${test_path#"$REPO_ROOT"/}"
    target="playwright:$relative_path"
    command=(env \
      API_BASE_URL="${API_BASE_URL:-http://127.0.0.1:8081}" \
      PLAYWRIGHT_APP_URL="${PLAYWRIGHT_APP_URL:-https://localhost:3000}" \
      PLAYWRIGHT_AGENT_APP_URL="${PLAYWRIGHT_AGENT_APP_URL:-https://localhost:3000}" \
      pnpm run test:playwright:all -- "$relative_path")
    if is_api_integration_spec "$source_path"; then
      run_target_with_retry "$target" \
        "${API_INTEGRATION_MAX_ATTEMPTS:-3}" \
        "${API_INTEGRATION_BACKOFF_SECONDS:-5}" \
        run_after_frontend_ready "${command[@]}" || return "$?"
    else
      run_target_or_stop "$target" "${command[@]}" || return "$?"
    fi
  done <"$files"
}

# Runs the typecheck, Rstest, and Playwright phases in order, stopping at the
# first failure so no later phase masks or delays the reported error.
# Example: if run_all_phases; then echo pass; else echo "$FAILED_TARGET"; fi
run_all_phases() {
  run_typecheck_phase || return "$?"
  run_rstest_phase || return "$?"
  run_playwright_phase || return "$?"
}