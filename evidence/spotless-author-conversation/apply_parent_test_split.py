"""Keep the external protocol test modules within the 50-line budget."""

from pathlib import Path

SOURCE = Path("/home/riley/A2A-Server-MCP/tests/test_forgejo_author_protocol.py")
TARGET = Path("/home/riley/A2A-Server-MCP/tests/test_forgejo_conversation_identity.py")
BLOCK = '''

@pytest.mark.parametrize('field', ['resume_session_id', 'author_provenance_id'])
def test_required_signed_continuity_fields_fail_closed(field):
    value = metadata()
    value[field] = ''
    with pytest.raises(ValueError):
        validate(value)
'''


def main() -> None:
    """Move continuity validation to the continuity-focused test module."""
    source = SOURCE.read_text()
    target = TARGET.read_text()
    if BLOCK in source:
        SOURCE.write_text(source.replace(BLOCK, '', 1).rstrip() + '\n')
    elif 'test_required_signed_continuity_fields_fail_closed' not in target:
        raise RuntimeError('continuity test source block is missing')
    if 'test_required_signed_continuity_fields_fail_closed' not in target:
        TARGET.write_text(target.rstrip() + BLOCK + '\n')


if __name__ == "__main__":
    main()