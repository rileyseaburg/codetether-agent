"""Write the external cross-replica lock test fixture."""

from pathlib import Path

PATH = Path("/home/riley/A2A-Server-MCP/tests/test_forgejo_author_lock.py")
CONTENT = '''from types import SimpleNamespace

import a2a_server
import pytest

from a2a_server.forgejo_author_lock import serialized
from tests.test_forgejo_author_protocol import metadata


@pytest.mark.asyncio
async def test_lock_fails_closed_when_database_is_unconfigured(monkeypatch):
    async def no_pool():
        return None

    monkeypatch.setitem(
        a2a_server.__dict__, 'database', SimpleNamespace(get_pool=no_pool)
    )
    with pytest.raises(RuntimeError, match='durable'):
        async with serialized(metadata()):
            pytest.fail('lock body must not run')


@pytest.mark.asyncio
async def test_lock_surrounds_the_full_creation_boundary(monkeypatch):
    events = []

    class Connection:
        async def execute(self, query, key):
            events.append(('lock' if 'unlock' not in query else 'unlock', key))

    class Pool:
        async def acquire(self):
            return Connection()

        async def release(self, _connection):
            events.append(('release', None))

    async def get_pool():
        return Pool()

    monkeypatch.setitem(
        a2a_server.__dict__, 'database', SimpleNamespace(get_pool=get_pool)
    )
    async with serialized(metadata()):
        events.append(('body', None))
    assert [event[0] for event in events] == ['lock', 'body', 'unlock', 'release']
    assert events[0][1] == events[2][1]
'''


def main() -> None:
    """Write the deterministic test source."""
    PATH.write_text(CONTENT)


if __name__ == "__main__":
    main()