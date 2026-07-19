"""Cryptographically bind canonical worker heartbeat liveness."""

from pathlib import Path

ROOT = Path('/home/riley/A2A-Server-MCP')
SERVER = ROOT / 'a2a_server'
TESTS = ROOT / 'tests'
MODULE_CONTENT = '''"""Canonical worker binding for heartbeat updates."""

from collections.abc import Mapping

from a2a_server.worker_registration_identity import bind as bind_identity


def bind(
    headers: Mapping[str, str],
    worker_id: str,
    worker: Mapping[str, object],
) -> dict[str, object]:
    """Verify canonical liveness and return trusted worker data."""
    value = dict(worker)
    name = str(value.get('name') or '')
    if not name.startswith('ctforgejo_'):
        return value
    capabilities, tenant_id = bind_identity(
        headers,
        worker_id,
        name,
        list(value.get('capabilities') or []),
        proof=('heartbeat', ''),
    )
    value['capabilities'] = capabilities
    value['tenant_id'] = tenant_id
    return value
'''
TEST_CONTENT = '''import pytest

from a2a_server.worker_heartbeat_identity import bind
from tests.forgejo_metadata import metadata
from tests.forgejo_provenance_fixture import registry
from tests.worker_identity_headers import headers


def test_canonical_heartbeat_requires_a_fresh_action_bound_proof(monkeypatch):
    value = metadata()
    name = str(value['target_agent_name'])
    worker = {'name': name, 'capabilities': []}
    monkeypatch.setenv('CODETETHER_PROVENANCE_SIGNING_KEYS', registry(value))
    proof = headers('heartbeat', 'worker-1', name, '')
    bound = bind(proof, 'worker-1', worker)
    assert bound['tenant_id'] == 'tenant'
    with pytest.raises(ValueError):
        bind(headers('register', 'worker-1', name, ''), 'worker-1', worker)
'''
ENDPOINT_TEST_CONTENT = '''from types import SimpleNamespace

import pytest

from a2a_server import monitor_api
from tests.forgejo_metadata import metadata
from tests.forgejo_provenance_fixture import registry
from tests.worker_identity_headers import headers


@pytest.mark.asyncio
async def test_canonical_heartbeat_endpoint_verifies_liveness(monkeypatch):
    value = metadata()
    name = str(value['target_agent_name'])
    worker = {'worker_id': 'worker-1', 'name': name, 'capabilities': []}

    async def updated(_worker_id: str) -> bool:
        return True

    async def no_op(_worker: dict[str, object]) -> None:
        return None

    monkeypatch.setenv('CODETETHER_PROVENANCE_SIGNING_KEYS', registry(value))
    monkeypatch.setattr(monitor_api.db, 'db_update_worker_heartbeat', updated)
    monkeypatch.setattr(monitor_api, '_redis_upsert_worker', no_op)
    monitor_api._registered_workers['worker-1'] = worker  # noqa: SLF001
    invalid = SimpleNamespace(
        headers=headers('register', 'worker-1', name, '')
    )
    valid = SimpleNamespace(
        headers=headers('heartbeat', 'worker-1', name, '')
    )
    try:
        with pytest.raises(monitor_api.HTTPException) as raised:
            await monitor_api.worker_heartbeat('worker-1', invalid)
        assert raised.value.status_code == 403  # noqa: PLR2004
        assert await monitor_api.worker_heartbeat('worker-1', valid) == {
            'success': True
        }
    finally:
        monitor_api._registered_workers.pop('worker-1', None)  # noqa: SLF001
'''


def main() -> None:
    """Install canonical heartbeat proof enforcement."""
    (SERVER / 'worker_heartbeat_identity.py').write_text(MODULE_CONTENT)
    registration = SERVER / 'worker_registration_identity.py'
    text = registration.read_text()
    old_signature = '''    capabilities: Sequence[str],
) -> tuple[list[str], str | None]:
'''
    new_signature = '''    capabilities: Sequence[str],
    *,
    action: str = 'register',
) -> tuple[list[str], str | None]:
'''
    if new_signature not in text:
        text = text.replace(old_signature, new_signature, 1)
    text = text.replace(
        "verify(headers, 'register', worker_id, name, '')",
        "verify(headers, action, worker_id, name, '')",
        1,
    )
    registration.write_text(text)
    monitor = SERVER / 'monitor_api.py'
    text = monitor.read_text()
    import_line = 'from .worker_heartbeat_identity import bind as bind_worker_heartbeat\n'
    if import_line not in text:
        anchor = 'from .worker_registration_identity import bind as bind_worker_identity\n'
        text = text.replace(anchor, anchor + import_line, 1)
    text = text.replace(
        'async def worker_heartbeat(worker_id: str):\n',
        'async def worker_heartbeat(worker_id: str, request: Request):\n',
        1,
    )
    start = '''    now = datetime.utcnow().isoformat()

    # Update in-memory cache
'''
    gate = '''    now = datetime.utcnow().isoformat()
    worker_info = _registered_workers.get(worker_id)
    if worker_info is None:
        worker_info = await db.db_get_worker(worker_id)
    if worker_info is None:
        worker_info = await _redis_get_worker(worker_id)
    try:
        if worker_info is not None:
            bind_worker_heartbeat(request.headers, worker_id, worker_info)
    except ValueError as error:
        raise HTTPException(
            status_code=403, detail=str(error)
        ) from error
    except RuntimeError as error:
        raise HTTPException(
            status_code=503, detail=str(error)
        ) from error

    # Update in-memory cache
'''
    if gate not in text:
        text = text.replace(start, gate, 1)
    sse_anchor = '''                        _registered_workers[worker_id] = worker_info
'''
    sse_gate = '''                        try:
                            worker_info = bind_worker_heartbeat(
                                request.headers, worker_id, worker_info
                            )
                        except ValueError as error:
                            raise HTTPException(
                                status_code=403, detail=str(error)
                            ) from error
                        except RuntimeError as error:
                            raise HTTPException(
                                status_code=503, detail=str(error)
                            ) from error
''' + sse_anchor
    if sse_gate not in text:
        text = text.replace(sse_anchor, sse_gate, 1)
    broad_catch = '''                        return {'success': True}
                except Exception as e:
'''
    typed_catch = '''                        return {'success': True}
                except HTTPException:
                    raise
                except Exception as e:
'''
    if typed_catch not in text:
        text = text.replace(broad_catch, typed_catch, 1)
    monitor.write_text(text)
    (TESTS / 'test_worker_heartbeat_identity.py').write_text(TEST_CONTENT)
    (TESTS / 'test_worker_heartbeat_endpoint.py').write_text(
        ENDPOINT_TEST_CONTENT
    )


if __name__ == '__main__':
    main()