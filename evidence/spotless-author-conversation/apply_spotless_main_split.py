"""Split the external review entrypoint into cohesive sub-50-line modules."""

from pathlib import Path

ROOT = Path("/home/riley/spotlessbinco")
DIR = ROOT / "scripts/codetether-review"
MAIN = ROOT / "scripts/codetether-pr-review.sh"
PLAYWRIGHT = ROOT / "tests/playwright/codetether/pr-review-workflow.spec.ts"
MODULES = '''#!/usr/bin/env bash

review_module_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$review_module_dir/common.sh"
source "$review_module_dir/context.sh"
source "$review_module_dir/diff.sh"
source "$review_module_dir/chunks.sh"
source "$review_module_dir/payload.sh"
source "$review_module_dir/chunk_pipeline.sh"
source "$review_module_dir/request_async.sh"
source "$review_module_dir/request_sync.sh"
source "$review_module_dir/response.sh"
source "$review_module_dir/forgejo.sh"
source "$review_module_dir/approval_state.sh"
source "$review_module_dir/approval.sh"
source "$review_module_dir/approval_credentials.sh"
source "$review_module_dir/author_signature.sh"
source "$review_module_dir/author_principal.sh"
source "$review_module_dir/author_state.sh"
source "$review_module_dir/author_identity.sh"
source "$review_module_dir/author_payload.sh"
source "$review_module_dir/task_create.sh"
source "$review_module_dir/author_delivery.sh"
source "$review_module_dir/review_bootstrap.sh"
source "$review_module_dir/review_orchestration.sh"
source "$review_module_dir/review_publish.sh"
unset review_module_dir
'''
BOOTSTRAP = '''#!/usr/bin/env bash

configure_review_environment() {
  CODETETHER_URL="${CODETETHER_JSON_URL:-}"
  if [[ -z "$CODETETHER_URL" && -n "${CODETETHER_API_URL:-}" ]]; then
    CODETETHER_URL="${CODETETHER_API_URL%/}/v1/json"
  fi
  if [[ -z "$CODETETHER_URL" || -z "${CODETETHER_TOKEN:-}" ]]; then
    echo "CodeTether review failed: API URL and token must be configured." >&2
    return 1
  fi
  require_command jq
  require_command curl
  require_command python3
  require_command forgejo-cli
  require_command sha256sum
  require_command timeout
  EVENT_PATH="${GITHUB_EVENT_PATH:-}"
  if [[ -z "$EVENT_PATH" || ! -f "$EVENT_PATH" ]]; then
    echo "GITHUB_EVENT_PATH is missing; cannot determine PR context." >&2
    return 1
  fi
}
'''
ORCHESTRATION = '''#!/usr/bin/env bash

run_review_pipeline() {
  load_review_context "$EVENT_PATH"
  if [[ -z "$PR_NUMBER" || -z "$REPO_FULL" ]]; then
    echo "Not a pull_request event or repository context is missing; skipping review."
    return 0
  fi
  load_forgejo_approval_credentials "$EVENT_PATH"
  CODETETHER_FORGEJO_APPROVAL_ACTOR="$approval_actor"
  log "Starting Forgejo PR review for ${REPO_FULL}#${PR_NUMBER}"
  resolve_commit_range
  temp_dir="$(mktemp -d)"
  trap 'rm -rf "$temp_dir"' EXIT
  diff_stat_file="$temp_dir/diff-stat.txt"
  diff_file="$temp_dir/diff.patch"
  normalized_file="$temp_dir/aggregate.json"
  comments_file="$temp_dir/comments.json"
  chunks_dir="$temp_dir/chunks"
  collect_review_diff "$BASE_SHA" "$HEAD_SHA" "$diff_stat_file" "$diff_file"
  manifest_file="$(prepare_review_chunks "$diff_file" "$chunks_dir")"
  run_chunk_review_pipeline "$manifest_file" "$diff_stat_file" "$temp_dir" "$normalized_file"
  review_markdown="$(extract_review_markdown "$normalized_file")" || {
    echo "CodeTether returned no markdown review field." >&2
    return 1
  }
  blocking_findings="$(extract_blocking_findings "$normalized_file")" || {
    echo "CodeTether returned no valid blocking_findings count." >&2
    return 1
  }
  publish_review_result
}
'''
PUBLISH = '''#!/usr/bin/env bash

publish_review_result() {
  local token="${FORGEJO_TOKEN:-}" api_base
  review_marker="<!-- codetether-pr-review:${PR_NUMBER} -->"
  review_file="$temp_dir/review.md"
  printf '%s\\n### CodeTether PR Review\\n\\n%s\\n\\n---\\n_Reviewed by CodeTether Agent for commit `%s`._\\n' \\
    "$review_marker" "$review_markdown" "${HEAD_SHA:0:12}" > "$review_file"
  if [[ -z "$token" ]]; then
    echo "Forgejo comment skipped: FORGEJO_TOKEN is not configured."
    cat "$review_file"
    return 0
  fi
  route_author_followup "$normalized_file" "$review_file" "$temp_dir"
  api_base="${FORGEJO_API_URL:-${GITHUB_SERVER_URL:-https://forgejo.invalid}/api/v1}"
  load_pr_comments "$api_base" "$REPO_FULL" "$PR_NUMBER" "$token" "$comments_file"
  upsert_review_comment "$api_base" "$REPO_FULL" "$PR_NUMBER" "$token" \\
    "$comments_file" "$review_marker" "$review_file" "$temp_dir/review-comment.json"
  approve_clean_review "$api_base" "$REPO_FULL" "$PR_NUMBER" "$HEAD_SHA" \\
    "$blocking_findings" "$approval_token" "$temp_dir"
  clear_forgejo_approval_credentials
  unset CODETETHER_FORGEJO_APPROVAL_ACTOR
  if (( blocking_findings == 0 )); then
    log "No blocking review findings; formal approval is current."
  else
    log "Blocking findings remain; PR was not approved."
  fi
}
'''
MAIN_CONTENT = '''#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/codetether-review/modules.sh"
configure_review_environment
run_review_pipeline
'''


def main() -> None:
    """Write focused modules and register them in source-contract tests."""
    (DIR / "modules.sh").write_text(MODULES)
    (DIR / "review_bootstrap.sh").write_text(BOOTSTRAP)
    (DIR / "review_orchestration.sh").write_text(ORCHESTRATION)
    (DIR / "review_publish.sh").write_text(PUBLISH)
    MAIN.write_text(MAIN_CONTENT)
    text = PLAYWRIGHT.read_text()
    anchor = '  "scripts/codetether-review/common.sh",\n'
    addition = '  "scripts/codetether-review/modules.sh",\n  "scripts/codetether-review/review_bootstrap.sh",\n  "scripts/codetether-review/review_orchestration.sh",\n  "scripts/codetether-review/review_publish.sh",\n'
    if addition.strip() not in text:
        if text.count(anchor) != 1:
            raise RuntimeError('Playwright module manifest anchor is missing')
        PLAYWRIGHT.write_text(text.replace(anchor, anchor + addition, 1))


if __name__ == "__main__":
    main()