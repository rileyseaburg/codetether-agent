"""Add a contract test for non-persisted Forgejo verification credentials."""

from pathlib import Path

ROOT = Path("/home/riley/spotlessbinco")
TEST = ROOT / "tests/codetether-review-task-create.test.sh"
WORKFLOW = ROOT / ".forgejo/workflows/codetether-pr-review.yml"
PLAYWRIGHT = ROOT / "tests/playwright/codetether/pr-review-workflow.spec.ts"
CONTENT = '''#!/usr/bin/env bash
set -euo pipefail
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$repo_root/scripts/codetether-review/common.sh"
source "$repo_root/scripts/codetether-review/task_create.sh"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
printf '{"metadata":{"safe":true}}' > "$tmp/payload.json"
FORGEJO_TOKEN=forgejo-scoped-token
curl() {
  printf '%s\\n' "$@" > "$tmp/arguments"
  local output=""
  while (($#)); do
    if [[ "$1" == -o ]]; then output="$2"; shift 2; else shift; fi
  done
  printf '{"id":"cttask_fixed"}' > "$output"
  printf '201'
}
task="$(create_codetether_task https://codetether.example/v1/agent/tasks \\
  codetether-token logical-key "$tmp/payload.json" "$tmp/response.json")"
[[ "$task" == cttask_fixed ]]
grep -Fx 'X-Forgejo-Token: forgejo-scoped-token' "$tmp/arguments" >/dev/null
grep -Fx 'Authorization: Bearer codetether-token' "$tmp/arguments" >/dev/null
! grep -F 'forgejo-scoped-token' "$tmp/payload.json" >/dev/null
echo "author task verification header tests passed"
'''


def insert(path: Path, anchor: str, addition: str) -> None:
    text = path.read_text()
    if addition.strip() in text:
        return
    if text.count(anchor) != 1:
        raise RuntimeError(f"task header test anchor missing: {path}")
    path.write_text(text.replace(anchor, anchor + addition, 1))


def main() -> None:
    """Write the shell test and connect CI/source assertions."""
    TEST.write_text(CONTENT)
    insert(
        WORKFLOW,
        '          bash tests/codetether-review-author-protocol.test.sh\n',
        '          bash tests/codetether-review-task-create.test.sh\n',
    )
    insert(
        PLAYWRIGHT,
        '    expect(script).toContain("/v1/agent/tasks");\n',
        '    expect(script).toContain("X-Forgejo-Token");\n',
    )


if __name__ == "__main__":
    main()