"""Bind canonical worker registration proofs to exact request bytes."""

from pathlib import Path

ROOT = Path('/home/riley/A2A-Server-MCP')
SERVER = ROOT / 'a2a_server'
TESTS = ROOT / 'tests'
LIFECYCLE_CONTENT = '''"""Canonical worker lifecycle authorization."""

from collections.abc import Mapping

from fastapi import HTTPException, Request

from a2a_server import database as db
from a2a_server.worker_registration_identity import bind
from a2a_server.worker_request_resource import derive


async def authorize(
    request: Request,
    worker_id: str,
    fallback: Mapping[str, object] | None,
) -> None:
    """Require key possession before removing a canonical worker."""
    try:
        worker = await db.db_get_worker(worker_id) or fallback
    except Exception as error:
        raise HTTPException(
            status_code=503, detail='Worker identity storage is unavailable'
        ) from error
    if not worker:
        return
    name = str(worker.get('name') or '')
    if not name.startswith('ctforgejo_'):
        return
    resource = await derive(request, worker_id)
    try:
        bind(
            request.headers,
            worker_id,
            name,
            list(worker.get('capabilities') or []),
            proof=('unregister', resource),
        )
    except ValueError as error:
        raise HTTPException(status_code=403, detail=str(error)) from error
    except RuntimeError as error:
        raise HTTPException(status_code=503, detail=str(error)) from error
'''
TEST_CONTENT = '''import hashlib

import pytest

from a2a_server import monitor_api
from tests.forgejo_metadata import metadata
from tests.forgejo_provenance_fixture import registry
from tests.worker_identity_headers import headers
from tests.worker_signed_request import SignedRequest


def signed_request(name: str, body: bytes = b'{}') -> SignedRequest:
    resource = f"worker-1:{hashlib.sha256(body).hexdigest()}"
    return SignedRequest(headers('register', 'worker-1', name, resource), body)


@pytest.mark.asyncio
async def test_registration_proof_is_bound_to_the_request_body(monkeypatch):
    value = metadata()
    name = str(value['target_agent_name'])

    async def persist(_worker: dict[str, object]) -> bool:
        return True

    async def no_op(*_args: object, **_kwargs: object) -> None:
        return None

    monkeypatch.setenv('CODETETHER_PROVENANCE_SIGNING_KEYS', registry(value))
    monkeypatch.setattr(monitor_api.db, 'db_upsert_worker', persist)
    monkeypatch.setattr(monitor_api, '_redis_upsert_worker', no_op)
    monkeypatch.setattr(monitor_api.monitoring_service, 'log_message', no_op)
    registration = monitor_api.WorkerRegistration(worker_id='worker-1', name=name)
    request = signed_request(name)
    try:
        await monitor_api.register_worker(registration, request)
        with pytest.raises(monitor_api.HTTPException) as raised:
            await monitor_api.register_worker(
                registration,
                SignedRequest(request.headers, b'{"tampered":true}'),
            )
        assert raised.value.status_code == 403  # noqa: PLR2004
    finally:
        monitor_api._registered_workers.pop('worker-1', None)  # noqa: SLF001
'''
UNREGISTER_TEST = '''import pytest

from a2a_server import monitor_api
from tests.forgejo_metadata import metadata
from tests.forgejo_provenance_fixture import registry
from tests.worker_identity_headers import headers
from tests.worker_signed_request import SignedRequest


@pytest.mark.asyncio
async def test_unregister_rejects_a_registration_proof(monkeypatch):
    value = metadata()
    name = str(value['target_agent_name'])

    async def worker(_worker_id: str) -> dict[str, object]:
        return {'name': name, 'capabilities': []}

    monkeypatch.setenv('CODETETHER_PROVENANCE_SIGNING_KEYS', registry(value))
    monkeypatch.setattr(monitor_api.db, 'db_get_worker', worker)
    request = SignedRequest(headers('register', 'worker-1', name, ''))
    with pytest.raises(monitor_api.HTTPException) as raised:
        await monitor_api.unregister_worker('worker-1', request)
    assert raised.value.status_code == 403  # noqa: PLR2004
'''


def main() -> None:
    """Install exact-body registration binding and focused coverage."""
    (SERVER / 'worker_lifecycle_authorization.py').write_text(LIFECYCLE_CONTENT)
    identity = SERVER / 'worker_registration_identity.py'
    text = identity.read_text()
    old_signature = '''    *,
    action: str = 'register',
    resource: str = '',
) -> tuple[list[str], str | None]:
'''
    new_signature = '''    *,
    proof: tuple[str, str] = ('register', ''),
) -> tuple[list[str], str | None]:
'''
    if new_signature not in text:
        text = text.replace(old_signature, new_signature, 1)
    text = text.replace(
        'verify(headers, action, worker_id, name, resource)',
        'verify(headers, proof[0], worker_id, name, proof[1])',
        1,
    )
    text = text.replace(
        "verify(headers, action, worker_id, name, '')",
        'verify(headers, proof[0], worker_id, name, proof[1])',
        1,
    )
    identity.write_text(text)
    monitor = SERVER / 'monitor_api.py'
    text = monitor.read_text()
    import_line = 'from .worker_request_resource import derive as worker_request_resource\n'
    if import_line not in text:
        anchor = 'from .worker_registration_identity import bind as bind_worker_identity\n'
        text = text.replace(anchor, anchor + import_line, 1)
    lifecycle_import = 'from .worker_lifecycle_authorization import authorize as authorize_worker_lifecycle\n'
    if lifecycle_import not in text:
        anchor = 'from .worker_registration_identity import bind as bind_worker_identity\n'
        text = text.replace(anchor, anchor + lifecycle_import, 1)
    call = '''        capabilities, tenant_id = bind_worker_identity(
            request.headers, registration.worker_id, registration.name, capabilities
        )
'''
    bound_call = '''        resource = await worker_request_resource(
            request, registration.worker_id
        )
        capabilities, tenant_id = bind_worker_identity(
            request.headers,
            registration.worker_id,
            registration.name,
            capabilities,
            proof=('register', resource),
        )
'''
    if bound_call not in text:
        text = text.replace(call, bound_call, 1)
    text = text.replace(
        '            resource=resource,\n',
        "            proof=('register', resource),\n",
        1,
    )
    text = text.replace(
        'async def unregister_worker(worker_id: str):\n',
        'async def unregister_worker(worker_id: str, request: Request):\n',
        1,
    )
    unregister_doc = '    """Unregister a worker."""\n'
    unregister_gate = unregister_doc + '''    await authorize_worker_lifecycle(
        request, worker_id, _registered_workers.get(worker_id)
    )
'''
    if unregister_gate not in text:
        text = text.replace(unregister_doc, unregister_gate, 1)
    monitor.write_text(text)
    (TESTS / 'test_worker_registration_body_proof.py').write_text(TEST_CONTENT)
    (TESTS / 'test_worker_unregister_proof.py').write_text(UNREGISTER_TEST)


if __name__ == '__main__':
    main()