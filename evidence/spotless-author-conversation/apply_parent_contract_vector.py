"""Add the shared Forgejo identity contract vector to parent tests."""

from pathlib import Path

PATH = Path("/home/riley/A2A-Server-MCP/tests/test_forgejo_author_protocol.py")
ANCHOR = """def test_identity_is_principal_bound():
"""
ADDITION = """def test_identity_matches_cross_runtime_contract_vector():
    assert canonical_identity('forge.example', 'alice', 'default') == (
        'ctforgejo_9e1fbe45a595b21bd9146db4a011cae0de38f193'
    )


"""


def main() -> None:
    """Insert the fixed vector exactly once."""
    text = PATH.read_text()
    if ADDITION.strip() in text:
        return
    if text.count(ANCHOR) != 1:
        raise RuntimeError("contract-vector anchor missing or ambiguous")
    PATH.write_text(text.replace(ANCHOR, ADDITION + ANCHOR, 1))


if __name__ == "__main__":
    main()