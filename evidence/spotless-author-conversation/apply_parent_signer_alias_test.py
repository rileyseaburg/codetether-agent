"""Cover Forgejo's alternate signer login field."""

from pathlib import Path

TRANSPORT = Path("/home/riley/A2A-Server-MCP/tests/forgejo_verification_transport.py")
TEST = Path("/home/riley/A2A-Server-MCP/tests/test_forgejo_author_verification.py")


def main() -> None:
    """Parameterize the mocked signer field and exercise its alias."""
    text = TRANSPORT.read_text()
    text = text.replace(
        "def transport(*, signer='alice', head=None, state='open', signed=None):\n",
        "def transport(*, signer='alice', signer_field='username', head=None, state='open', signed=None):\n",
    )
    text = text.replace(
        "'signer': {'username': signer},\n",
        "'signer': {signer_field: signer},\n",
    )
    TRANSPORT.write_text(text)
    test = TEST.read_text()
    addition = '''

@pytest.mark.asyncio
async def test_server_accepts_forgejo_login_signer_alias():
    await verify(metadata(), 'scoped-token', transport(signer_field='login'))
'''
    if 'test_server_accepts_forgejo_login_signer_alias' not in test:
        anchor = '''@pytest.mark.asyncio
@pytest.mark.parametrize('change', ['signer', 'head', 'state'])
'''
        if test.count(anchor) != 1:
            raise RuntimeError("signer alias test anchor is missing")
        test = test.replace(anchor, addition + '\n' + anchor, 1)
        TEST.write_text(test)


if __name__ == "__main__":
    main()