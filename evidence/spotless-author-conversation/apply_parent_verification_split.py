"""Split Forgejo response parsing from verification orchestration."""

from pathlib import Path

ROOT = Path("/home/riley/A2A-Server-MCP/a2a_server")
VERIFY = ROOT / "forgejo_author_verification.py"
RESPONSE = ROOT / "forgejo_verification_response.py"
RESPONSE_CONTENT = '''"""Typed response parsing for Forgejo verification."""

import httpx


async def response_json(
    client: httpx.AsyncClient, url: str
) -> dict[str, object]:
    """Fetch one required Forgejo object or fail closed."""
    response = await client.get(url)
    response.raise_for_status()
    value = response.json()
    if not isinstance(value, dict):
        raise ValueError('Forgejo verification response is invalid')
    return value


def nested(value: object, *keys: str) -> object:
    """Read a nested response field without loose dynamic typing."""
    for key in keys:
        if not isinstance(value, dict):
            return None
        value = value.get(key)
    return value
'''
OLD_HELPERS = '''

async def _json(client: httpx.AsyncClient, url: str) -> dict[str, object]:
    response = await client.get(url)
    response.raise_for_status()
    value = response.json()
    if not isinstance(value, dict):
        raise ValueError('Forgejo verification response is invalid')
    return value


def _nested(value: object, *keys: str) -> object:
    for key in keys:
        if not isinstance(value, dict):
            return None
        value = value.get(key)
    return value
'''


def main() -> None:
    """Write the parser and update imports and calls."""
    RESPONSE.write_text(RESPONSE_CONTENT)
    text = VERIFY.read_text()
    if OLD_HELPERS in text:
        text = text.replace(OLD_HELPERS, '', 1)
    text = text.replace(
        'from .forgejo_verification_config import api_base\n',
        'from .forgejo_verification_config import api_base\nfrom .forgejo_verification_response import nested, response_json\n',
    )
    text = text.replace('_json(', 'response_json(').replace('_nested(', 'nested(')
    VERIFY.write_text(text)


if __name__ == "__main__":
    main()