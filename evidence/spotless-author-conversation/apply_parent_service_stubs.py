"""Extract behavior-neutral author service test stubs."""

from pathlib import Path

ROOT = Path('/home/riley/A2A-Server-MCP/tests')
STUBS = ROOT / 'forgejo_service_stubs.py'
TEST = ROOT / 'test_forgejo_author_service.py'
STUBS_CONTENT = '''"""Ordered collaborators for author service tests."""

from collections.abc import AsyncIterator
from contextlib import asynccontextmanager
from types import SimpleNamespace
from typing import Protocol

from pytest import MonkeyPatch

import a2a_server.forgejo_author_service as service
import a2a_server.forgejo_author_authenticate as authentication

from tests.forgejo_service_fixture import provenance_key


class Validator(Protocol):
    """Worker validator accepted by the service under test."""
    async def __call__(self, metadata: object, *, strict: bool) -> None: ...


def install(monkeypatch: MonkeyPatch, events: list[str]) -> Validator:
    """Install recording collaborators and return the worker validator."""
    @asynccontextmanager
    async def gate(_metadata: object) -> AsyncIterator[None]:
        events.append('lock')
        yield
        events.append('unlock')

    async def prepare(_metadata: object) -> tuple[str, None]:
        return 'cttask_fixed', None

    async def validate(_metadata: object, *, strict: bool) -> None:
        assert strict is True
        events.append('validate')

    async def verify(_metadata: object, token: str) -> SimpleNamespace:
        assert token == 'forgejo-token'
        events.append('verify')
        return provenance_key()

    monkeypatch.setattr(service, 'serialized', gate)
    monkeypatch.setattr(service, 'prepare', prepare)
    monkeypatch.setattr(authentication, 'verify', verify)
    return validate
'''


def main() -> None:
    """Write stubs and simplify the service behavior test."""
    STUBS.write_text(STUBS_CONTENT)
    text = TEST.read_text()
    text = text.replace('from contextlib import asynccontextmanager\n\n', '')
    text = text.replace('    provenance_key,\n', '')
    if 'from tests.forgejo_service_stubs import install\n' not in text:
        anchor = ')\n\n\n@pytest.mark.asyncio\n'
        text = text.replace(
            anchor,
            ')\nfrom tests.forgejo_service_stubs import install\n\n\n@pytest.mark.asyncio\n',
            1,
        )
    start = text.find('    @asynccontextmanager\n')
    end = text.find('    task = await service.create(\n', start)
    if start >= 0 and end >= 0:
        text = text[:start] + '    validate = install(monkeypatch, events)\n' + text[end:]
    TEST.write_text(text)


if __name__ == '__main__':
    main()