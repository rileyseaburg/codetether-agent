"""Reject credential-bearing or ambiguous Forgejo allowlist URLs."""

from pathlib import Path

PATH = Path("/home/riley/A2A-Server-MCP/a2a_server/forgejo_verification_config.py")
OLD = '''    if parsed.scheme != 'https' or parsed.hostname != host.lower():
        raise RuntimeError('Forgejo verification endpoint is unsafe')
'''
NEW = '''    unsafe = (
        parsed.scheme != 'https'
        or parsed.hostname != host.lower()
        or parsed.username is not None
        or parsed.password is not None
        or bool(parsed.query)
        or bool(parsed.fragment)
    )
    if unsafe:
        raise RuntimeError('Forgejo verification endpoint is unsafe')
'''


def main() -> None:
    """Extend the URL safety predicate exactly once."""
    text = PATH.read_text()
    if NEW in text:
        return
    if text.count(OLD) != 1:
        raise RuntimeError("Forgejo URL safety anchor is missing")
    PATH.write_text(text.replace(OLD, NEW, 1))


if __name__ == "__main__":
    main()