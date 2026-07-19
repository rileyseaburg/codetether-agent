"""Align the workflow idempotency header with the server-derived work key."""

from pathlib import Path

PATH = Path("/home/riley/spotlessbinco/scripts/codetether-review/author_payload.sh")
OLD = """author_work_key() {
  printf 'forgejo-pr-review:%s:%s:forgejo-author-review:%s\\n' \\
    "$REPO_FULL" "$PR_NUMBER" "$HEAD_SHA"
}
"""
NEW = """author_work_key() {
  printf 'forgejo-pr-review:v1:%s:%s:%s:forgejo-author-review:%s\\n' \\
    "$AUTHOR_FORGEJO_HOST" "$REPO_FULL" "$PR_NUMBER" "$HEAD_SHA"
}
"""


def main() -> None:
    """Replace exactly one legacy work-key formatter."""
    text = PATH.read_text()
    if NEW in text:
        return
    if text.count(OLD) != 1:
        raise RuntimeError("author work-key anchor missing or ambiguous")
    PATH.write_text(text.replace(OLD, NEW, 1))


if __name__ == "__main__":
    main()