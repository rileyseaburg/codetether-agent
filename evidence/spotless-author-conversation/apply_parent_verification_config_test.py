"""Write Forgejo allowlist safety tests for the external server."""

from pathlib import Path

PATH = Path("/home/riley/A2A-Server-MCP/tests/test_forgejo_verification_config.py")
CONTENT = '''import json

import pytest

from a2a_server.forgejo_verification_config import api_base


def test_configured_https_host_is_accepted(monkeypatch):
    value = {'forge.example': 'https://forge.example/api/v1'}
    monkeypatch.setenv('CODETETHER_FORGEJO_API_BASE_URLS', json.dumps(value))
    assert api_base('forge.example') == 'https://forge.example/api/v1'


def test_unconfigured_host_fails_closed(monkeypatch):
    monkeypatch.setenv('CODETETHER_FORGEJO_API_BASE_URLS', '{}')
    with pytest.raises(RuntimeError, match='not configured'):
        api_base('forge.example')


@pytest.mark.parametrize(
    'url',
    [
        'http://forge.example/api/v1',
        'https://user@forge.example/api/v1',
        'https://forge.example/api/v1?target=other',
    ],
)
def test_ambiguous_endpoint_is_rejected(monkeypatch, url):
    value = {'forge.example': url}
    monkeypatch.setenv('CODETETHER_FORGEJO_API_BASE_URLS', json.dumps(value))
    with pytest.raises(RuntimeError, match='unsafe'):
        api_base('forge.example')
'''


def main() -> None:
    """Write the deterministic allowlist test source."""
    PATH.write_text(CONTENT)


if __name__ == "__main__":
    main()