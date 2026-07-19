"""Keep Forgejo author tasks on the task-bound claim protocol."""

from pathlib import Path

ROOT = Path('/home/riley/A2A-Server-MCP')
SERVER = ROOT / 'a2a_server'
TESTS = ROOT / 'tests'
MODULE_CONTENT = '''"""Identity policy for legacy extended worker claims."""

from fastapi import HTTPException

from a2a_server import database as db

PREFIX = 'ctforgejo_'


async def resolve(worker_id: str, requested_name: str | None) -> str:
    """Return the durable generic identity or reject protocol bypasses."""
    try:
        worker = await db.db_get_worker(worker_id)
    except Exception as error:
        raise HTTPException(
            status_code=503, detail='Worker identity storage is unavailable'
        ) from error
    if not worker:
        raise HTTPException(status_code=409, detail='Worker is not registered')
    name = str(worker.get('name') or '')
    if not name:
        raise HTTPException(status_code=409, detail='Worker identity is missing')
    if requested_name and requested_name != name:
        raise HTTPException(
            status_code=403, detail='Requested agent does not match the worker identity'
        )
    if name.startswith(PREFIX):
        raise HTTPException(
            status_code=409,
            detail='Canonical author workers require task-bound claims',
        )
    return name
'''
TEST_CONTENT = '''import pytest
from fastapi import HTTPException

from a2a_server import database
from a2a_server.worker_extended_claim import resolve


@pytest.mark.asyncio
async def test_extended_claim_rejects_canonical_workers(monkeypatch):
    async def worker(_worker_id: str) -> dict[str, object]:
        return {'name': 'ctforgejo_canonical'}

    monkeypatch.setattr(database, 'db_get_worker', worker)
    with pytest.raises(HTTPException) as raised:
        await resolve('worker-1', 'ctforgejo_canonical')
    assert raised.value.status_code == 409  # noqa: PLR2004


@pytest.mark.asyncio
async def test_extended_claim_uses_the_durable_generic_name(monkeypatch):
    async def worker(_worker_id: str) -> dict[str, object]:
        return {'name': 'generic-worker'}

    monkeypatch.setattr(database, 'db_get_worker', worker)
    assert await resolve('worker-1', None) == 'generic-worker'
    with pytest.raises(HTTPException) as raised:
        await resolve('worker-1', 'ctforgejo_spoofed')
    assert raised.value.status_code == 403  # noqa: PLR2004
'''


def main() -> None:
    """Install the extended-claim identity guard and focused tests."""
    (SERVER / 'worker_extended_claim.py').write_text(MODULE_CONTENT)
    routes = SERVER / 'worker_progress_routes.py'
    text = routes.read_text()
    import_line = 'from .worker_extended_claim import resolve as resolve_extended_claim_name\n'
    if import_line not in text:
        anchor = 'from .worker_auth import verify_auth\n'
        text = text.replace(anchor, anchor + import_line, 1)
    modern_anchor = '''    from .persistent_worker_pool import claim_extended_task as _do_claim

    result = await _do_claim(
        worker_id=resolved_worker_id,
        agent_name=x_agent_name,
    )
'''
    modern_guard = '''    from .persistent_worker_pool import claim_extended_task as _do_claim

    agent_name = await resolve_extended_claim_name(
        resolved_worker_id, x_agent_name
    )
    result = await _do_claim(
        worker_id=resolved_worker_id,
        agent_name=agent_name,
    )
'''
    if modern_guard not in text:
        text = text.replace(modern_anchor, modern_guard, 1)
    legacy_anchor = '''    from .persistent_worker_pool import claim_extended_task

    result = await claim_extended_task(
        worker_id=claim.worker_id,
        agent_name=claim.agent_name,
'''
    legacy_guard = '''    from .persistent_worker_pool import claim_extended_task

    agent_name = await resolve_extended_claim_name(
        claim.worker_id, claim.agent_name
    )
    result = await claim_extended_task(
        worker_id=claim.worker_id,
        agent_name=agent_name,
'''
    if legacy_guard not in text:
        text = text.replace(legacy_anchor, legacy_guard, 1)
    routes.write_text(text)
    (TESTS / 'test_worker_extended_claim_guard.py').write_text(TEST_CONTENT)


if __name__ == '__main__':
    main()