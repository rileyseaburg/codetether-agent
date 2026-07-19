"""Prevent generic upsert fallback from reopening verified author tasks."""

import re

from pathlib import Path

DATABASE = Path('/home/riley/A2A-Server-MCP/a2a_server/database.py')
TEST = Path('/home/riley/A2A-Server-MCP/tests/test_forgejo_idempotent_upsert.py')


def main() -> None:
    """Make verified protocol rows immutable on task-ID conflicts."""
    text = DATABASE.read_text()
    if "IS DISTINCT FROM 'codetether.forgejo-author.v1'" not in text:
        pattern = r"(?m)^(\s*)tenant_id = COALESCE\(EXCLUDED\.tenant_id, tasks\.tenant_id\)$"
        replacement = (
            r"\1tenant_id = COALESCE(EXCLUDED.tenant_id, tasks.tenant_id)\n"
            r"\1WHERE tasks.metadata->>'protocol'\n"
            r"\1    IS DISTINCT FROM 'codetether.forgejo-author.v1'"
        )
        text, count = re.subn(pattern, replacement, text)
        if count != 2:
            raise RuntimeError('expected both task upsert clauses')
        DATABASE.write_text(text)
    TEST.write_text('''from pathlib import Path

EXPECTED_CLAUSES = 2


def test_verified_author_task_conflicts_do_not_reopen_existing_work():
    source = Path('a2a_server/database.py').read_text()
    assert source.count("WHERE tasks.metadata->>'protocol'") == EXPECTED_CLAUSES
    assert source.count("IS DISTINCT FROM 'codetether.forgejo-author.v1'") == EXPECTED_CLAUSES
''')


if __name__ == '__main__':
    main()