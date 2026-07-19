"""Remove the last repository-specific reviewer fallback and split approval."""

from pathlib import Path

ROOT = Path("/home/riley/spotlessbinco")
APPROVAL = ROOT / "scripts/codetether-review/approval.sh"
INPUT = ROOT / "scripts/codetether-review/approval_input.sh"
CREATE = ROOT / "scripts/codetether-review/approval_create.sh"
MODULES = ROOT / "scripts/codetether-review/modules.sh"
TEST = ROOT / "tests/codetether-review-approval.test.sh"
PLAYWRIGHT = ROOT / "tests/playwright/codetether/pr-review-workflow.spec.ts"
INPUT_CONTENT = '''#!/usr/bin/env bash

validate_approval_inputs() {
  local token="$1" blocking="$2" actor="$3"
  if [[ -z "$token" ]]; then
    echo "A Forgejo approval credential is required when auto-approval is enabled." >&2
    return 1
  fi
  if [[ ! "$blocking" =~ ^[0-9]+$ ]]; then
    echo "Invalid blocking finding count for approval: $blocking" >&2
    return 1
  fi
  if [[ ! "$actor" =~ ^[A-Za-z0-9._-]{1,128}$ ]]; then
    echo "A configured Forgejo approval actor is required." >&2
    return 1
  fi
}
'''
CREATE_CONTENT = '''#!/usr/bin/env bash

create_codetether_approval() {
  local api="$1" repo="$2" pr="$3" head="$4" marker="$5" actor="$6"
  local token="$7" payload="$8" response="$9" body
  body="$marker
CodeTether found zero blocking findings for commit \`${head:0:12}\`."
  jq -n --arg body "$body" --arg commit_id "$head" \\
    '{body:$body,event:"APPROVED",commit_id:$commit_id}' > "$payload"
  forgejo_request POST "${api%/}/repos/$repo/pulls/$pr/reviews" \\
    "$token" "$payload" "$response" || return 1
  jq -e --arg head "$head" --arg marker "$marker" --arg actor "$actor" '
    ((.state // "") | ascii_upcase) == "APPROVED"
      and ((.official // false) == true) and (.commit_id // "") == $head
      and ((.body // "") | contains($marker)) and (.user.login // "") == $actor
  ' "$response" >/dev/null || {
    echo "Forgejo did not confirm an approval for commit $head." >&2
    return 1
  }
  log "Forgejo approval accepted for commit ${head:0:12}."
}
'''


def insert(path: Path, anchor: str, addition: str) -> None:
    text = path.read_text()
    if addition.strip() in text:
        return
    if text.count(anchor) != 1:
        raise RuntimeError(f"approval split anchor missing: {path}")
    path.write_text(text.replace(anchor, anchor + addition, 1))


def main() -> None:
    """Write focused helpers and convert the approval orchestrator."""
    INPUT.write_text(INPUT_CONTENT)
    CREATE.write_text(CREATE_CONTENT)
    text = APPROVAL.read_text()
    old_guard = '''  if [[ -z "$token" ]]; then
    echo "A Forgejo approval credential is required when auto-approval is enabled." >&2; return 1
  fi
  if [[ ! "$blocking" =~ ^[0-9]+$ ]]; then
    echo "Invalid blocking finding count for approval: $blocking" >&2; return 1
  fi
'''
    new_guard = '''  actor="${CODETETHER_FORGEJO_APPROVAL_ACTOR:-}"
  validate_approval_inputs "$token" "$blocking" "$actor" || return 1
'''
    if new_guard not in text:
        if text.count(old_guard) != 1:
            raise RuntimeError('approval input block missing')
        text = text.replace(old_guard, new_guard, 1)
    text = text.replace(
        '  actor="${CODETETHER_FORGEJO_APPROVAL_ACTOR:-spotless-agent}"\n', '', 1
    )
    start = text.find('  local body\n')
    end = text.rfind('\n}')
    call = '''  create_codetether_approval "$api" "$repo" "$pr" "$reviewed_head" \\
    "$marker" "$actor" "$token" "$payload" "$response"
'''
    if start >= 0:
        text = text[:start] + call + text[end:]
    APPROVAL.write_text(text)
    insert(
        MODULES,
        'source "$review_module_dir/approval_state.sh"\n',
        'source "$review_module_dir/approval_input.sh"\nsource "$review_module_dir/approval_create.sh"\n',
    )
    insert(TEST, 'source "$repo_root/scripts/codetether-review/approval_state.sh"\n',
        'source "$repo_root/scripts/codetether-review/approval_input.sh"\nsource "$repo_root/scripts/codetether-review/approval_create.sh"\n')
    test = TEST.read_text().replace(
        '  unset CODETETHER_FORGEJO_APPROVAL_ACTOR\n',
        '  export CODETETHER_FORGEJO_APPROVAL_ACTOR=review-agent\n',
    ).replace('spotless-agent', 'review-agent')
    TEST.write_text(test)
    insert(PLAYWRIGHT, '  "scripts/codetether-review/approval_state.sh",\n',
        '  "scripts/codetether-review/approval_input.sh",\n  "scripts/codetether-review/approval_create.sh",\n')


if __name__ == "__main__":
    main()