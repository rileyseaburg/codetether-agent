"""Add cross-runtime and cross-revision invariants to the shell contract test."""

from pathlib import Path

PATH = Path("/home/riley/spotlessbinco/tests/codetether-review-author-protocol.test.sh")
ANCHOR = '''identity="$(canonical_agent_identity forge.example alice default)"
'''
ADDITION = '''[[ "$identity" == "ctforgejo_9e1fbe45a595b21bd9146db4a011cae0de38f193" ]]
'''
SECOND_ANCHOR = '''[[ "$AUTHOR_PROVENANCE_VERIFIED" == true ]]
[[ "$AUTHOR_AGENT_ID" == "$identity" ]]
'''
SECOND_ADDITION = '''first_context="$(author_conversation_id)"
first_work_key="$(author_work_key)"
original_head="$HEAD_SHA"
HEAD_SHA="$(printf 'f%.0s' {1..40})"
[[ "$(author_conversation_id)" == "$first_context" ]]
[[ "$(author_work_key)" != "$first_work_key" ]]
HEAD_SHA="$original_head"
'''


def insert(anchor: str, addition: str) -> None:
    text = PATH.read_text()
    if addition.strip() in text:
        return
    if text.count(anchor) != 1:
        raise RuntimeError("protocol-test anchor missing or ambiguous")
    PATH.write_text(text.replace(anchor, anchor + addition, 1))


def main() -> None:
    """Install fixed-vector and continuity assertions."""
    insert(ANCHOR, ADDITION)
    insert(SECOND_ANCHOR, SECOND_ADDITION)


if __name__ == "__main__":
    main()