"""Split external author identity parsing into cohesive shell modules."""

from pathlib import Path

ROOT = Path("/home/riley/spotlessbinco")
IDENTITY = ROOT / "scripts/codetether-review/author_identity.sh"
PRINCIPAL = ROOT / "scripts/codetether-review/author_principal.sh"
STATE = ROOT / "scripts/codetether-review/author_state.sh"
MAIN = ROOT / "scripts/codetether-pr-review.sh"
TEST = ROOT / "tests/codetether-review-author-protocol.test.sh"
PLAYWRIGHT = ROOT / "tests/playwright/codetether/pr-review-workflow.spec.ts"
PRINCIPAL_CONTENT = '''#!/usr/bin/env bash

commit_trailer() {
  git show -s --format=%B "$1" | git interpret-trailers --parse |
    sed -n "s/^$2:[[:space:]]*//p" | tail -n 1
}

canonical_agent_identity() {
  local host="${1,,}" login="${2,,}" slot="${3,,}" digest
  [[ "$host" =~ ^[a-z0-9][a-z0-9.:-]{0,127}$ ]] || return 1
  [[ "$login" =~ ^[a-z0-9][a-z0-9._-]{0,127}$ ]] || return 1
  [[ "$slot" =~ ^[a-z0-9][a-z0-9._-]{0,127}$ ]] || return 1
  digest="$(printf '%s\\n%s\\n%s' "$host" "$login" "$slot" | sha256sum)"
  printf 'ctforgejo_%s\\n' "${digest:0:40}"
}
'''
STATE_CONTENT = '''#!/usr/bin/env bash

clear_author_identity() {
  AUTHOR_AGENT_ID=""
  AUTHOR_SESSION_ID=""
  AUTHOR_PROVENANCE_ID=""
  AUTHOR_GIT_SIGNER=""
  AUTHOR_FORGEJO_HOST=""
  AUTHOR_FORGEJO_LOGIN=""
  AUTHOR_AGENT_SLOT=""
  AUTHOR_PROVENANCE_VERIFIED=false
}
'''


def insert(path: Path, anchor: str, addition: str) -> None:
    text = path.read_text()
    if addition.strip() in text:
        return
    if text.count(anchor) != 1:
        raise RuntimeError(f"identity split anchor missing: {path}")
    path.write_text(text.replace(anchor, anchor + addition, 1))


def main() -> None:
    """Write focused modules and update every trusted source manifest."""
    text = IDENTITY.read_text()
    marker = 'load_author_identity() {'
    if text.count(marker) != 1:
        raise RuntimeError('author identity loader anchor is missing')
    IDENTITY.write_text('#!/usr/bin/env bash\n\n' + marker + text.split(marker, 1)[1])
    PRINCIPAL.write_text(PRINCIPAL_CONTENT)
    STATE.write_text(STATE_CONTENT)
    insert(
        MAIN,
        'source "$script_dir/codetether-review/author_signature.sh"\n',
        'source "$script_dir/codetether-review/author_principal.sh"\nsource "$script_dir/codetether-review/author_state.sh"\n',
    )
    insert(
        TEST,
        'source "$repo_root/scripts/codetether-review/author_signature.sh"\n',
        'source "$repo_root/scripts/codetether-review/author_principal.sh"\nsource "$repo_root/scripts/codetether-review/author_state.sh"\n',
    )
    insert(
        PLAYWRIGHT,
        '  "scripts/codetether-review/author_signature.sh",\n',
        '  "scripts/codetether-review/author_principal.sh",\n  "scripts/codetether-review/author_state.sh",\n',
    )


if __name__ == "__main__":
    main()