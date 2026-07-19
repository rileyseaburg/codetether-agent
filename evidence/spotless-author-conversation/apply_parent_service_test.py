"""Write the external atomic author-service tests."""

from pathlib import Path

PATH = Path("/home/riley/A2A-Server-MCP/tests/test_forgejo_author_service.py")
CONTENT = '''from contextlib import asynccontextmanager
from types import SimpleNamespace

import pytest

import a2a_server.forgejo_author_service as service


@pytest.mark.asyncio
async def test_create_holds_gate_and_requires_persistence(monkeypatch):
    events = []

    @asynccontextmanager
    async def gate(_metadata):
        events.append('lock')
        yield
        events.append('unlock')

    async def prepare(_metadata):
        return 'cttask_fixed', None

    async def validate(_metadata, *, strict):
        assert strict is True
        events.append('validate')

    async def verify(_metadata, token):
        assert token == 'forgejo-token'
        events.append('verify')

    class Bridge:
        async def create_task(self, **kwargs):
            events.append('create')
            return kwargs

    monkeypatch.setattr(service, 'serialized', gate)
    monkeypatch.setattr(service, 'prepare', prepare)
    monkeypatch.setattr(service, 'verify', verify)
    task = await service.create(
        Bridge(),
        SimpleNamespace(title='review', prompt='data', agent_type='build', priority=7),
        {'model': 'test'},
        SimpleNamespace(model_ref='provider/model'),
        'global',
        validate,
        'forgejo-token',
    )
    assert task['task_id'] == 'cttask_fixed'
    assert task['require_persistence'] is True
    assert 'forgejo-token' not in str(task)
    assert events == ['verify', 'lock', 'validate', 'create', 'unlock']
'''


def main() -> None:
    """Write the deterministic test source."""
    PATH.write_text(CONTENT)


if __name__ == "__main__":
    main()