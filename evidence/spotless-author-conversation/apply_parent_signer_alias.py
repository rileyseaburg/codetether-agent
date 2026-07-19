"""Accept documented Forgejo signer login field variants."""

from pathlib import Path

PATH = Path("/home/riley/A2A-Server-MCP/a2a_server/forgejo_author_verification.py")
OLD = "    signer = nested(commit, 'commit', 'verification', 'signer', 'username')\n"
NEW = '''    signer = nested(commit, 'commit', 'verification', 'signer', 'username')
    signer = signer or nested(commit, 'commit', 'verification', 'signer', 'login')
'''


def main() -> None:
    """Add the Forgejo login alias without weakening verification."""
    text = PATH.read_text()
    if NEW in text:
        return
    if text.count(OLD) != 1:
        raise RuntimeError("Forgejo signer field anchor is missing")
    PATH.write_text(text.replace(OLD, NEW, 1))


if __name__ == "__main__":
    main()