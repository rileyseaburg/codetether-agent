"""Split helpers approaching the hard 50-line ceiling."""

from pathlib import Path

ROOT = Path("/home/riley/spotlessbinco")
DIR = ROOT / "scripts/codetether-review"
MODULES = DIR / "modules.sh"
SOURCE = ROOT / "tests/playwright/codetether/pr-review-source.ts"


def insert(path: Path, anchor: str, addition: str) -> None:
    text = path.read_text()
    if addition.strip() in text:
        return
    if text.count(anchor) != 1:
        raise RuntimeError(f"proactive split anchor missing: {path}")
    path.write_text(text.replace(anchor, anchor + addition, 1))


def split_approval_guard() -> None:
    """Move PR freshness validation out of approval reconciliation."""
    guard = DIR / 'approval_pr_guard.sh'
    guard.write_text('''#!/usr/bin/env bash

validate_approval_pr() {
  local api="$1" repo="$2" pr="$3" reviewed_head="$4" token="$5" output="$6"
  local current_head head_repo base_repo
  forgejo_request GET "${api%/}/repos/$repo/pulls/$pr" "$token" "" "$output" || return 1
  if ! jq -e '.state == "open" and ((.merged // false) == false)' "$output" >/dev/null; then
    log "Skipping Forgejo approval because PR $pr is not open."
    return 2
  fi
  current_head="$(jq -er '.head.sha | select(type == "string" and length > 0)' "$output")"
  head_repo="$(jq -er '.head.repo.full_name | select(type == "string" and length > 0)' "$output")"
  base_repo="$(jq -er '.base.repo.full_name | select(type == "string" and length > 0)' "$output")"
  if [[ "$head_repo" != "$base_repo" ]]; then
    log "Skipping Forgejo approval for cross-repository PR $pr."
    return 2
  fi
  if [[ "$current_head" != "$reviewed_head" ]]; then
    echo "Refusing stale approval: reviewed=$reviewed_head current=$current_head." >&2
    return 1
  fi
}
''')
    path = DIR / 'approval.sh'
    text = path.read_text()
    start = text.find('  forgejo_request GET "${api%/}/repos/$repo/pulls/$pr"')
    stop = text.find('\n\n  marker=', start)
    if start >= 0 and stop >= 0:
        replacement = '''  guard_status=0
  validate_approval_pr "$api" "$repo" "$pr" "$reviewed_head" \\
    "$token" "$pr_file" || guard_status=$?
  case "$guard_status" in 0) ;; 2) return 0 ;; *) return 1 ;; esac'''
        text = text[:start] + replacement + text[stop:]
    text = text.replace(
        '  local current_head head_repo base_repo marker active_id dismissed_id\n',
        '  local marker active_id dismissed_id guard_status\n',
    )
    declaration = '  local marker active_id dismissed_id guard_status\n'
    if declaration not in text:
        anchor = '  local token="$6" temp_dir="$7" enabled pr_file reviews_file payload response actor\n'
        if text.count(anchor) != 1:
            raise RuntimeError('approval local-variable anchor is missing')
        text = text.replace(anchor, anchor + declaration, 1)
    path.write_text(text)
    insert(MODULES, 'source "$review_module_dir/approval_input.sh"\n',
        'source "$review_module_dir/approval_pr_guard.sh"\n')
    test = ROOT / 'tests/codetether-review-approval.test.sh'
    insert(test, 'source "$repo_root/scripts/codetether-review/approval_input.sh"\n',
        'source "$repo_root/scripts/codetether-review/approval_pr_guard.sh"\n')
    insert(SOURCE, '  "scripts/codetether-review/approval_input.sh",\n',
        '  "scripts/codetether-review/approval_pr_guard.sh",\n')


def split_approval_candidate() -> None:
    """Move configured reviewer selection out of Vault credential loading."""
    candidate = DIR / 'approval_candidate.sh'
    candidate.write_text('''#!/usr/bin/env bash

select_approval_candidate() {
  local author="$1" candidates="$2" entry actor path entries
  approval_actor=""
  vault_path=""
  IFS=',' read -ra entries <<< "$candidates"
  for entry in "${entries[@]}"; do
    [[ "$entry" == *=* ]] || continue
    actor="${entry%%=*}"; path="${entry#*=}"
    [[ "$actor" =~ ^[A-Za-z0-9._-]{1,128}$ ]] || continue
    [[ "$path" =~ ^[A-Za-z0-9_./-]{1,256}$ ]] || continue
    if [[ "${actor,,}" != "${author,,}" ]]; then
      approval_actor="$actor"; vault_path="$path"; return 0
    fi
  done
  echo "No configured Forgejo reviewer is independent of $author." >&2
  return 1
}
''')
    path = DIR / 'approval_credentials.sh'
    text = path.read_text()
    start = text.find('  candidates="${CODETETHER_REVIEWER_CANDIDATES:-}"')
    stop = text.find('  approval_token="$(vault kv get', start)
    if start >= 0 and stop >= 0:
        replacement = '''  candidates="${CODETETHER_REVIEWER_CANDIDATES:-}"
  select_approval_candidate "$author" "$candidates" || return 1
'''
        text = text[:start] + replacement + text[stop:]
    path.write_text(text)
    insert(MODULES, 'source "$review_module_dir/approval_credentials.sh"\n',
        'source "$review_module_dir/approval_candidate.sh"\n')
    test = ROOT / 'tests/codetether-review-approval-credentials.test.sh'
    insert(test, 'repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"\n',
        'source "$repo_root/scripts/codetether-review/approval_candidate.sh"\n')
    insert(SOURCE, '  "scripts/codetether-review/approval_credentials.sh",\n',
        '  "scripts/codetether-review/approval_candidate.sh",\n')


def split_async_poll() -> None:
    """Separate task creation from terminal-state polling."""
    path = DIR / 'request_async.sh'
    poll = DIR / 'request_poll.sh'
    text = path.read_text()
    marker = '  task_status=""\n'
    end = text.rfind('\n}')
    if text.count(marker) == 1 and end >= 0:
        before, loop = text.split(marker, 1)
        poll.write_text('''#!/usr/bin/env bash

poll_review_task() {
  local url="$1" token="$2" task_id="$3" response_file="$4"
  local attempts="$5" interval="$6" connect_timeout="$7" request_timeout="$8"
  local task_status="" status attempt
''' + loop[:loop.rfind('\n}')].replace('  ', '  ', 1) + '\n}\n')
        call = '''  poll_review_task "$url" "$token" "$task_id" "$response_file" \\
    "$attempts" "$interval" "$connect_timeout" "$request_timeout"
'''
        path.write_text(before + call + '}\n')
    elif not poll.exists():
        raise RuntimeError('async polling split is incomplete')
    path.write_text(
        path.read_text().replace(
            '  local status task_id task_status attempt\n', '  local status task_id\n'
        )
    )
    insert(MODULES, 'source "$review_module_dir/request_async.sh"\n',
        'source "$review_module_dir/request_poll.sh"\n')
    insert(SOURCE, '  "scripts/codetether-review/request_async.sh",\n',
        '  "scripts/codetether-review/request_poll.sh",\n')


def main() -> None:
    """Apply all proactive SRP splits."""
    split_approval_guard()
    split_approval_candidate()
    split_async_poll()


if __name__ == '__main__':
    main()