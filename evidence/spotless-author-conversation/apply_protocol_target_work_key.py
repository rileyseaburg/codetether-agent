"""Bind client and server work keys to the canonical author route."""

from pathlib import Path

CLIENT = Path("/home/riley/spotlessbinco/scripts/codetether-review/author_payload.sh")
SERVER = Path("/home/riley/A2A-Server-MCP/a2a_server/forgejo_author_task.py")
CLIENT_OLD = '''  printf 'forgejo-pr-review:v1:%s:%s:%s:forgejo-author-review:%s\\n' \\
    "$AUTHOR_FORGEJO_HOST" "${REPO_FULL,,}" "$PR_NUMBER" "$HEAD_SHA"
'''
CLIENT_NEW = '''  printf 'forgejo-pr-review:v1:%s:%s:%s:forgejo-author-review:%s:%s\\n' \\
    "$AUTHOR_FORGEJO_HOST" "${REPO_FULL,,}" "$PR_NUMBER" "$HEAD_SHA" \\
    "$AUTHOR_AGENT_ID"
'''
SERVER_OLD = '''        str(metadata['pr_head_sha']).lower(),
    )
'''
SERVER_NEW = '''        str(metadata['pr_head_sha']).lower(),
        str(metadata['target_agent_name']),
    )
'''


def replace(path: Path, old: str, new: str) -> None:
    text = path.read_text()
    if new in text:
        return
    if text.count(old) != 1:
        raise RuntimeError(f"target work-key anchor missing: {path}")
    path.write_text(text.replace(old, new, 1))


def main() -> None:
    """Apply the canonical-target work namespace at both ends."""
    replace(CLIENT, CLIENT_OLD, CLIENT_NEW)
    replace(SERVER, SERVER_OLD, SERVER_NEW)


if __name__ == "__main__":
    main()