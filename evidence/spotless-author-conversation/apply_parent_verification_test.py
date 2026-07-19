"""Write independent Forgejo verification tests for the external server."""

from pathlib import Path

PATH = Path("/home/riley/A2A-Server-MCP/tests/test_forgejo_author_verification.py")
CONTENT = '''import json

import httpx
import pytest

from a2a_server.forgejo_author_verification import verify
from tests.test_forgejo_author_protocol import metadata


def transport(*, signer='alice', head=None, state='open'):
    expected = 'a' * 40

    def respond(request):
        assert request.headers['authorization'] == 'token scoped-token'
        if '/pulls/' in request.url.path:
            value = {
                'state': state,
                'head': {'sha': head or expected},
                'user': {'login': 'alice'},
            }
        else:
            value = {
                'sha': expected,
                'commit': {'verification': {
                    'verified': True, 'signer': {'username': signer}
                }},
            }
        return httpx.Response(200, json=value)

    return httpx.MockTransport(respond)


@pytest.fixture(autouse=True)
def configured_host(monkeypatch):
    value = {'forge.example': 'https://forge.example/api/v1'}
    monkeypatch.setenv('CODETETHER_FORGEJO_API_BASE_URLS', json.dumps(value))


@pytest.mark.asyncio
async def test_server_independently_verifies_pr_and_signer():
    await verify(metadata(), 'scoped-token', transport())


@pytest.mark.asyncio
@pytest.mark.parametrize('change', ['signer', 'head', 'state'])
async def test_server_rejects_stale_or_unverified_proof(change):
    kwargs = {change: {'signer': 'mallory', 'head': 'b' * 40, 'state': 'closed'}[change]}
    with pytest.raises((ValueError, LookupError)):
        await verify(metadata(), 'scoped-token', transport(**kwargs))


@pytest.mark.asyncio
async def test_server_requires_a_non_persisted_verification_token():
    with pytest.raises(RuntimeError, match='credential'):
        await verify(metadata(), '', transport())
'''


def main() -> None:
    """Write the deterministic verification test source."""
    PATH.write_text(CONTENT)


if __name__ == "__main__":
    main()