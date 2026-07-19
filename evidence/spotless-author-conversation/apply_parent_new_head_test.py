"""Keep both signed head aliases coherent in the task identity test."""

from pathlib import Path

PATH = Path("/home/riley/A2A-Server-MCP/tests/test_forgejo_author_protocol.py")
OLD = """    second['pr_head_sha'] = 'b' * 40
    assert task_identity(first) != task_identity(second)
"""
NEW = """    second['pr_head_sha'] = 'b' * 40
    second['head_sha'] = 'b' * 40
    assert task_identity(first) != task_identity(second)
"""


def main() -> None:
    """Insert the corresponding head alias exactly once."""
    text = PATH.read_text()
    if NEW in text:
        return
    if text.count(OLD) != 1:
        raise RuntimeError("new-head test anchor missing or ambiguous")
    PATH.write_text(text.replace(OLD, NEW, 1))


if __name__ == "__main__":
    main()