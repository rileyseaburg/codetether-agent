"""Write a concurrent-retry convergence test for the external service."""

from pathlib import Path

PATH = Path("/home/riley/A2A-Server-MCP/tests/test_forgejo_author_concurrency.py")
CONTENT = '''import asyncio
from contextlib import asynccontextmanager
from types import SimpleNamespace

import pytest

import a2a_server.forgejo_author_service as service


@pytest.mark.asyncio
async def test_concurrent_retries_create_one_durable_task(monkeypatch):
    lock = asyncio.Lock()
    state = {'task': None, 'creates': 0}

    @asynccontextmanager
    async def serialized(_metadata):
        async with lock:
            yield

    async def prepare(_metadata):
        return 'cttask_fixed', state['task']

    async def validate(_metadata, *, strict):
        assert strict is True

    async def verify(_metadata, _token):
        return None

    class Bridge:
        async def create_task(self, **_kwargs):
            state['creates'] += 1
            await asyncio.sleep(0)
            state['task'] = {'id': 'cttask_fixed'}
            return state['task']

    monkeypatch.setattr(service, 'serialized', serialized)
    monkeypatch.setattr(service, 'prepare', prepare)
    monkeypatch.setattr(service, 'verify', verify)
    task_data = SimpleNamespace(
        title='review', prompt='data', agent_type='build', priority=1
    )
    route = SimpleNamespace(model_ref=None)
    calls = [
        service.create(Bridge(), task_data, {}, route, 'global', validate, 'token')
        for _ in range(2)
    ]
    results = await asyncio.gather(*calls)
    assert state['creates'] == 1
    assert results == [{'id': 'cttask_fixed'}, {'id': 'cttask_fixed'}]
'''


def main() -> None:
    """Write the deterministic concurrency test source."""
    PATH.write_text(CONTENT)


if __name__ == "__main__":
    main()