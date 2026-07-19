"""Update approval wiring assertions after the SRP module split."""

from pathlib import Path

PATH = Path("/home/riley/spotlessbinco/tests/codetether-review-approval.test.sh")
OLD = '''orchestrator="$repo_root/scripts/codetether-pr-review.sh"
grep -F 'FORGEJO_TOKEN: ${{ github.token }}' "$workflow" >/dev/null
! grep -F '${{ secrets.FORGEJO_TOKEN }}' "$workflow" >/dev/null
grep -F 'load_forgejo_approval_credentials "$EVENT_PATH"' "$orchestrator" >/dev/null
grep -F 'CODETETHER_FORGEJO_APPROVAL_ACTOR="$approval_actor"' "$orchestrator" >/dev/null
grep -F 'clear_forgejo_approval_credentials' "$orchestrator" >/dev/null
grep -F '"$blocking_findings" "$approval_token" "$temp_dir"' "$orchestrator" >/dev/null
'''
NEW = '''orchestration="$repo_root/scripts/codetether-review/review_orchestration.sh"
publish="$repo_root/scripts/codetether-review/review_publish.sh"
grep -F 'FORGEJO_TOKEN: ${{ github.token }}' "$workflow" >/dev/null
! grep -F '${{ secrets.FORGEJO_TOKEN }}' "$workflow" >/dev/null
grep -F 'load_forgejo_approval_credentials "$EVENT_PATH"' "$orchestration" >/dev/null
grep -F 'CODETETHER_FORGEJO_APPROVAL_ACTOR="$approval_actor"' "$orchestration" >/dev/null
grep -F 'clear_forgejo_approval_credentials' "$publish" >/dev/null
grep -F '"$blocking_findings" "$approval_token" "$temp_dir"' "$publish" >/dev/null
'''


def main() -> None:
    """Replace the obsolete monolithic-entrypoint assertions."""
    text = PATH.read_text()
    if NEW in text:
        return
    if text.count(OLD) != 1:
        raise RuntimeError("approval wiring assertion block is missing")
    PATH.write_text(text.replace(OLD, NEW, 1))


if __name__ == "__main__":
    main()