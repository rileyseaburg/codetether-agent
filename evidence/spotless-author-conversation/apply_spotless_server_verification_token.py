"""Forward the event-scoped Forgejo token only as a verification header."""

from pathlib import Path

PATH = Path("/home/riley/spotlessbinco/scripts/codetether-review/task_create.sh")
OLD = '''    -H 'Content-Type: application/json' -H "Idempotency-Key: $key" \\
    --data-binary "@$payload")"
'''
NEW = '''    -H 'Content-Type: application/json' -H "Idempotency-Key: $key" \\
    -H "X-Forgejo-Token: $FORGEJO_TOKEN" --data-binary "@$payload")"
'''


def main() -> None:
    """Add the non-persisted Forgejo verification credential header."""
    text = PATH.read_text()
    if NEW in text:
        return
    if text.count(OLD) != 1:
        raise RuntimeError("task verification header anchor is missing")
    PATH.write_text(text.replace(OLD, NEW, 1))


if __name__ == "__main__":
    main()