"""Make canonical worker claims atomic across API replicas."""

from pathlib import Path

ROOT = Path('/home/riley/A2A-Server-MCP')
SERVER = ROOT / 'a2a_server'
TESTS = ROOT / 'tests'
MODULE_CONTENT = '''"""Durable cross-replica reservation for verified worker claims."""

from a2a_server import database as db

ACQUIRED = 'acquired'
OWNED = 'owned'
UNAVAILABLE = 'unavailable'


async def reserve(task_id: str, worker_id: str) -> str:
    """Atomically reserve a pending task or recognize an idempotent retry."""
    try:
        pool = await db.get_pool()
        if not pool:
            raise RuntimeError('Task claim storage is unavailable')
        async with pool.acquire() as connection:
            result = await connection.execute(
                "UPDATE tasks SET status = 'running', worker_id = $2, "
                "started_at = COALESCE(started_at, NOW()), updated_at = NOW() "
                "WHERE id = $1 AND status = 'pending' AND worker_id IS NULL",
                task_id,
                worker_id,
            )
            if 'UPDATE 1' in result:
                return ACQUIRED
            row = await connection.fetchrow(
                'SELECT status, worker_id FROM tasks WHERE id = $1', task_id
            )
    except RuntimeError:
        raise
    except Exception as error:
        raise RuntimeError('Task claim storage is unavailable') from error
    if row and row['status'] == 'running' and row['worker_id'] == worker_id:
        return OWNED
    return UNAVAILABLE
'''
ENDPOINT_TEST = '''from types import SimpleNamespace

import pytest

from a2a_server import worker_sse


class Registry:
    def __init__(self) -> None:
        self.released = False

    async def claim_task(self, _task_id: str, _worker_id: str) -> bool:
        return True

    async def release_task(self, _task_id: str, _worker_id: str) -> bool:
        self.released = True
        return True


@pytest.mark.asyncio
async def test_claim_endpoint_rolls_back_a_lost_durable_race(monkeypatch):
    registry = Registry()

    async def verified(*_args: object, **_kwargs: object) -> bool:
        return True

    async def unavailable(*_args: object) -> str:
        return 'unavailable'

    monkeypatch.setattr(worker_sse, '_verify_auth', lambda _request: None)
    monkeypatch.setattr(worker_sse, 'require_forgejo_worker_claim', verified)
    monkeypatch.setattr(worker_sse, 'reserve_forgejo_claim', unavailable)
    monkeypatch.setattr(worker_sse, 'get_worker_registry', lambda: registry)
    request = SimpleNamespace(headers={})
    claim = worker_sse.TaskClaimRequest(task_id='cttask_1')
    with pytest.raises(worker_sse.HTTPException) as raised:
        await worker_sse.claim_task(
            request, claim, worker_id='worker-1', x_worker_id=None
        )
    assert raised.value.status_code == 409  # noqa: PLR2004
    assert registry.released
'''
TEST_CONTENT = '''import pytest

from a2a_server import forgejo_claim_reservation as reservation


class Connection:
    def __init__(self) -> None:
        self.status = 'pending'
        self.worker_id = None

    async def execute(self, _query: str, _task_id: str, worker_id: str) -> str:
        if self.status == 'pending' and self.worker_id is None:
            self.status = 'running'
            self.worker_id = worker_id
            return 'UPDATE 1'
        return 'UPDATE 0'

    async def fetchrow(self, _query: str, _task_id: str) -> dict[str, object]:
        return {'status': self.status, 'worker_id': self.worker_id}


class Context:
    def __init__(self, connection: Connection) -> None:
        self.connection = connection

    async def __aenter__(self) -> Connection:
        return self.connection

    async def __aexit__(self, *_args: object) -> None:
        return None


class Pool:
    def __init__(self) -> None:
        self.connection = Connection()

    def acquire(self) -> Context:
        return Context(self.connection)


@pytest.mark.asyncio
async def test_only_one_worker_can_reserve_a_verified_task(monkeypatch):
    pool = Pool()
    monkeypatch.setattr(reservation.db, 'get_pool', lambda: async_value(pool))
    assert await reservation.reserve('cttask_1', 'worker-1') == 'acquired'
    assert await reservation.reserve('cttask_1', 'worker-2') == 'unavailable'
    assert await reservation.reserve('cttask_1', 'worker-1') == 'owned'


async def async_value(value: object) -> object:
    return value
'''


def main() -> None:
    """Install durable claim reservation and its concurrency regression test."""
    (SERVER / 'forgejo_claim_reservation.py').write_text(MODULE_CONTENT)
    claim = SERVER / 'forgejo_worker_claim.py'
    text = claim.read_text()
    text = text.replace(') -> None:\n    """Require the target worker key', ') -> bool:\n    """Require the target worker key', 1)
    text = text.replace(
        "            raise LookupError('verified task does not exist in durable storage')\n        return\n",
        "            raise LookupError('verified task does not exist in durable storage')\n        return False\n",
        1,
    )
    text = text.replace(
        "    if not isinstance(metadata, dict) or metadata.get('protocol') != PROTOCOL:\n        return\n",
        "    if not isinstance(metadata, dict) or metadata.get('protocol') != PROTOCOL:\n        return False\n",
        1,
    )
    final = "    if metadata.get('tenant_id') != key.tenant_id:\n        raise ValueError('worker proof tenant does not match the author task')\n"
    if final + '    return True\n' not in text:
        text = text.replace(final, final + '    return True\n', 1)
    claim.write_text(text)
    sse = SERVER / 'worker_sse.py'
    text = sse.read_text()
    import_line = 'from .forgejo_claim_reservation import reserve as reserve_forgejo_claim\n'
    if import_line not in text:
        anchor = 'from .forgejo_worker_claim import require as require_forgejo_worker_claim\n'
        text = text.replace(anchor, anchor + import_line, 1)
    text = text.replace(
        '        await require_forgejo_worker_claim(\n',
        '        verified_task = await require_forgejo_worker_claim(\n',
        1,
    )
    claim_call = '    success = await registry.claim_task(claim.task_id, resolved_worker_id)\n'
    reservation = claim_call + '''    if success and verified_task:
        try:
            state = await reserve_forgejo_claim(
                claim.task_id, resolved_worker_id
            )
        except RuntimeError as error:
            await registry.release_task(claim.task_id, resolved_worker_id)
            raise HTTPException(status_code=503, detail=str(error)) from error
        if state == 'unavailable':
            await registry.release_task(claim.task_id, resolved_worker_id)
            success = False
'''
    if reservation not in text:
        text = text.replace(claim_call, reservation, 1)
    sse.write_text(text)
    (TESTS / 'test_forgejo_claim_reservation.py').write_text(TEST_CONTENT)
    (TESTS / 'test_forgejo_claim_reservation_endpoint.py').write_text(ENDPOINT_TEST)


if __name__ == '__main__':
    main()