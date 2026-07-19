"""Complete the structural task-data mutation contract."""

from pathlib import Path

PATH = Path("/home/riley/A2A-Server-MCP/a2a_server/forgejo_author_types.py")


def main() -> None:
    """Add fields the HTTP-independent service writes to task data."""
    text = PATH.read_text()
    future = 'from __future__ import annotations\n\n'
    if future not in text:
        marker = '"""Structural types used by the Forgejo author task service."""\n\n'
        if text.count(marker) != 1:
            raise RuntimeError('author type module anchor is missing')
        text = text.replace(marker, marker + future, 1)
    fields = '''    metadata: MutableMapping[str, object]
    routing: RoutingDecision
'''
    if fields not in text:
        anchor = '    priority: int\n'
        if text.count(anchor) != 1:
            raise RuntimeError('task data field anchor is missing')
        text = text.replace(anchor, anchor + fields, 1)
    PATH.write_text(text)


if __name__ == '__main__':
    main()