"""Separate durable-read failure from author task preparation behavior."""

from pathlib import Path

ROOT = Path('/home/riley/A2A-Server-MCP/tests')
PREPARE = ROOT / 'test_forgejo_author_prepare.py'
FAILURE = ROOT / 'test_forgejo_task_read_failure.py'
CONTENT = '''import pytest

from a2a_server import database
from a2a_server.forgejo_author_task import prepare
from tests.forgejo_metadata import metadata


@pytest.mark.asyncio
async def test_prepare_propagates_durable_read_failures(monkeypatch):
    async def get_pool():
        return object()

    async def failed_read(_task_id):
        raise OSError('database read failed')

    monkeypatch.setattr(database, 'get_pool', get_pool)
    monkeypatch.setattr(database, 'db_get_task', failed_read)
    with pytest.raises(OSError, match='database read failed'):
        await prepare(metadata())
'''


def main() -> None:
    """Move the independent storage-failure test into its own module."""
    text = PREPARE.read_text()
    marker = '\n\n@pytest.mark.asyncio\nasync def test_prepare_propagates_durable_read_failures'
    index = text.find(marker)
    if index >= 0:
        PREPARE.write_text(text[:index] + '\n')
    FAILURE.write_text(CONTENT)


if __name__ == '__main__':
    main()