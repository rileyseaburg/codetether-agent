"""Harden continuity and failure cleanup in the external workflow."""

from pathlib import Path

PAYLOAD = Path("/home/riley/spotlessbinco/scripts/codetether-review/author_payload.sh")
IDENTITY = Path("/home/riley/spotlessbinco/scripts/codetether-review/author_identity.sh")
MAIN = Path("/home/riley/spotlessbinco/scripts/codetether-pr-review.sh")


def replace_once(path: Path, old: str, new: str) -> None:
    text = path.read_text()
    if new in text:
        return
    if text.count(old) != 1:
        raise RuntimeError(f"anchor missing or ambiguous: {path}")
    path.write_text(text.replace(old, new, 1))


def main() -> None:
    """Apply the continuity, cleanup, and dependency invariants."""
    replace_once(
        PAYLOAD,
        '''  digest="$(printf '%s\\n%s\\n%s\\n%s' "$REPO_FULL" "$PR_NUMBER" "$HEAD_SHA" \\
    "$AUTHOR_AGENT_ID" | sha256sum)"
''',
        '''  digest="$(printf '%s\\n%s\\n%s' "$REPO_FULL" "$PR_NUMBER" \\
    "$AUTHOR_AGENT_ID" | sha256sum)"
''',
    )
    replace_once(
        IDENTITY,
        '''  expected_identity="$(canonical_agent_identity \\
    "$AUTHOR_FORGEJO_HOST" "$AUTHOR_FORGEJO_LOGIN" "$AUTHOR_AGENT_SLOT")" || return 0
''',
        '''  expected_identity="$(canonical_agent_identity \\
    "$AUTHOR_FORGEJO_HOST" "$AUTHOR_FORGEJO_LOGIN" "$AUTHOR_AGENT_SLOT")" || {
    log "Author delivery disabled: signed Forgejo principal is unsafe."
    clear_author_identity
    return 0
  }
''',
    )
    replace_once(
        MAIN,
        '''require_command jq
require_command curl
require_command python3
''',
        '''require_command jq
require_command curl
require_command python3
require_command forgejo-cli
require_command sha256sum
require_command timeout
''',
    )
    replace_once(
        PAYLOAD,
        '''author_conversation_id() {
  local digest
  digest="$(printf '%s\\n%s\\n%s' "$REPO_FULL" "$PR_NUMBER" \\
    "$AUTHOR_AGENT_ID" | sha256sum)"
''',
        '''author_conversation_id() {
  local digest repo="${REPO_FULL,,}"
  digest="$(printf '%s\\n%s\\n%s' "$repo" "$PR_NUMBER" \\
    "$AUTHOR_AGENT_ID" | sha256sum)"
''',
    )
    replace_once(
        PAYLOAD,
        '''    "$AUTHOR_FORGEJO_HOST" "$REPO_FULL" "$PR_NUMBER" "$HEAD_SHA"
''',
        '''    "$AUTHOR_FORGEJO_HOST" "${REPO_FULL,,}" "$PR_NUMBER" "$HEAD_SHA"
''',
    )


if __name__ == "__main__":
    main()