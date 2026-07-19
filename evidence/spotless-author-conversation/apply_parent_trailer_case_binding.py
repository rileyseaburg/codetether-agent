"""Keep opaque signed identifiers case-sensitive during trailer binding."""

from pathlib import Path

PATH = Path("/home/riley/A2A-Server-MCP/a2a_server/forgejo_commit_trailers.py")
OLD = '''        expected = str(metadata.get(field) or '')
        if trailers[label].lower() != expected.lower():
            raise ValueError(f'signed trailer does not match {field}')
'''
NEW = '''        expected = str(metadata.get(field) or '')
        signed = trailers[label]
        principal_field = field in {'forgejo_host', 'forgejo_author_login', 'agent_slot'}
        matches = signed.lower() == expected.lower() if principal_field else signed == expected
        if not matches:
            raise ValueError(f'signed trailer does not match {field}')
'''


def main() -> None:
    """Replace uniform lowercasing with field-aware comparison."""
    text = PATH.read_text()
    if NEW in text:
        return
    if text.count(OLD) != 1:
        raise RuntimeError("signed trailer comparison anchor is missing")
    PATH.write_text(text.replace(OLD, NEW, 1))


if __name__ == "__main__":
    main()