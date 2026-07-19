"""Install the external stale-head guard before author task creation."""

from pathlib import Path

ROOT = Path("/home/riley/spotlessbinco")
GUARD = ROOT / "scripts/codetether-review/author_guard.sh"
MODULES = ROOT / "scripts/codetether-review/modules.sh"
PUBLISH = ROOT / "scripts/codetether-review/review_publish.sh"
WORKFLOW = ROOT / ".forgejo/workflows/codetether-pr-review.yml"
PLAYWRIGHT = ROOT / "tests/playwright/codetether/pr-review-workflow.spec.ts"
TEST = ROOT / "tests/codetether-review-author-guard.test.sh"
GUARD_CONTENT = '''#!/usr/bin/env bash

require_current_author_review_head() {
  local api="$1" token="$2" output="$3" current_head state
  forgejo_request GET "$api/repos/$REPO_FULL/pulls/$PR_NUMBER" \\
    "$token" "" "$output" || return 1
  current_head="$(jq -r '.head.sha // empty' "$output")"
  state="$(jq -r '.state // empty' "$output")"
  if [[ "$state" != open || "$current_head" != "$HEAD_SHA" ]]; then
    echo "Refusing stale author delivery: reviewed=$HEAD_SHA current=$current_head state=$state." >&2
    return 1
  fi
}
'''
TEST_CONTENT = '''#!/usr/bin/env bash
set -euo pipefail
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$repo_root/scripts/codetether-review/author_guard.sh"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
REPO_FULL=owner/repo
PR_NUMBER=42
HEAD_SHA="$(printf 'a%.0s' {1..40})"
response_head="$HEAD_SHA"
response_state=open
forgejo_request() {
  jq -n --arg head "$response_head" --arg state "$response_state" \\
    '{head:{sha:$head},state:$state}' > "$5"
}
require_current_author_review_head https://forge.example/api/v1 token "$tmp/current.json"
response_head="$(printf 'b%.0s' {1..40})"
if require_current_author_review_head api token "$tmp/stale.json" 2>/dev/null; then
  echo "FAIL: stale author review head was accepted" >&2
  exit 1
fi
response_head="$HEAD_SHA"
response_state=closed
if require_current_author_review_head api token "$tmp/closed.json" 2>/dev/null; then
  echo "FAIL: closed PR was accepted" >&2
  exit 1
fi
echo "author delivery freshness tests passed"
'''


def insert(path: Path, anchor: str, addition: str) -> None:
    text = path.read_text()
    if addition.strip() in text:
        return
    if text.count(anchor) != 1:
        raise RuntimeError(f"author guard anchor missing: {path}")
    path.write_text(text.replace(anchor, anchor + addition, 1))


def main() -> None:
    """Write the guard and connect it to workflow execution and tests."""
    GUARD.write_text(GUARD_CONTENT)
    TEST.write_text(TEST_CONTENT)
    insert(
        MODULES,
        'source "$review_module_dir/author_signature.sh"\n',
        'source "$review_module_dir/author_guard.sh"\n',
    )
    publish = PUBLISH.read_text()
    old = '''  route_author_followup "$normalized_file" "$review_file" "$temp_dir"
  api_base="${FORGEJO_API_URL:-${GITHUB_SERVER_URL:-https://forgejo.invalid}/api/v1}"
'''
    new = '''  api_base="${FORGEJO_API_URL:-${GITHUB_SERVER_URL:-https://forgejo.invalid}/api/v1}"
  require_current_author_review_head "$api_base" "$token" "$temp_dir/current-pr.json"
  route_author_followup "$normalized_file" "$review_file" "$temp_dir"
'''
    if new not in publish:
        if publish.count(old) != 1:
            raise RuntimeError('publish freshness anchor missing')
        PUBLISH.write_text(publish.replace(old, new, 1))
    insert(
        WORKFLOW,
        '          bash tests/codetether-review-author-protocol.test.sh\n',
        '          bash tests/codetether-review-author-guard.test.sh\n',
    )
    insert(
        PLAYWRIGHT,
        '  "scripts/codetether-review/author_signature.sh",\n',
        '  "scripts/codetether-review/author_guard.sh",\n',
    )


if __name__ == "__main__":
    main()