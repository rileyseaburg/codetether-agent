"""Add authenticated-tenant isolation regression tests."""

from pathlib import Path

PREPARE = Path("/home/riley/A2A-Server-MCP/tests/test_forgejo_author_prepare.py")
TEST = Path("/home/riley/A2A-Server-MCP/tests/test_forgejo_tenant_binding.py")
CONTENT = '''from types import SimpleNamespace

import pytest

import a2a_server
from a2a_server.forgejo_author_task import prepare, task_identity
from a2a_server.forgejo_request_scope import resolve
from tests.test_forgejo_author_protocol import metadata


def test_authenticated_tenants_have_distinct_task_identities():
    first, second = metadata(), metadata()
    first['idempotency_scope'] = 'tenant:first'
    second['idempotency_scope'] = 'tenant:second'
    assert task_identity(first) != task_identity(second)


def test_request_tenant_overrides_client_scope():
    request = SimpleNamespace(
        state=SimpleNamespace(policy_user={'tenant_id': 'tenant-a', 'id': 'user'})
    )
    assert resolve(request) == ('tenant:tenant-a', 'tenant-a')


@pytest.mark.asyncio
async def test_worker_lookup_is_tenant_scoped(monkeypatch):
    seen = []

    async def get_pool():
        return object()

    async def get_task(_task_id):
        return None

    async def get_worker(_name, *, tenant_id=None):
        seen.append(tenant_id)
        return {'worker_id': 'worker-1'}

    database = SimpleNamespace(
        get_pool=get_pool,
        db_get_task=get_task,
        db_get_active_worker_by_name=get_worker,
    )
    monkeypatch.setitem(a2a_server.__dict__, 'database', database)
    value = metadata()
    value['tenant_id'] = 'tenant-a'
    await prepare(value)
    assert seen == ['tenant-a']
'''


def main() -> None:
    """Update the existing fake and write tenant-isolation tests."""
    text = PREPARE.read_text()
    text = text.replace(
        '    async def get_worker(_name):\n',
        '    async def get_worker(_name, *, tenant_id=None):\n',
    )
    PREPARE.write_text(text)
    TEST.write_text(CONTENT)


if __name__ == "__main__":
    main()