"""Add focused canonical-worker possession and routing tests."""

from pathlib import Path

ROOT = Path('/home/riley/A2A-Server-MCP/tests')


def write(name: str, content: str) -> None:
    """Write one focused test module."""
    (ROOT / name).write_text(content)


def main() -> None:
    """Install worker proof fixtures and fail-closed tests."""
    write('worker_identity_headers.py', '''"""Signed canonical-worker request headers for tests."""

import hashlib
import hmac
import time

from a2a_server.worker_identity_proof_payload import canonical

KEY_ID = 'author-key'
SECRET = 'test-provenance-secret'


def headers(action: str, worker_id: str, name: str, resource: str) -> dict[str, str]:
    """Sign one fresh server-compatible worker possession proof."""
    timestamp = str(int(time.time()))
    payload = canonical(action, worker_id, name, resource, timestamp)
    signature = hmac.new(SECRET.encode(), payload, hashlib.sha256).hexdigest()
    return {
        'x-codetether-key-id': KEY_ID,
        'x-codetether-proof-timestamp': timestamp,
        'x-codetether-worker-proof': signature,
    }
''')
    write('test_worker_identity_proof.py', '''import json

import pytest

from a2a_server.worker_identity_proof import verify
from tests.forgejo_metadata import metadata
from tests.forgejo_provenance_fixture import registry
from tests.worker_identity_headers import headers


def test_matches_the_rust_worker_proof_vector(monkeypatch):
    monkeypatch.setattr('tests.worker_identity_headers.time.time', lambda: 1700000000)
    proof = headers(
        'claim', 'worker-1', 'ctforgejo_author', 'cttask_1'
    )
    assert proof['x-codetether-worker-proof'] == (
        'a54f3fd301714efa72dd0be12b5558e9e5d552a8aefac3189955e567ed0814ce'
    )


def test_worker_proof_binds_request_identity_and_resource(monkeypatch):
    value = metadata()
    name = str(value['target_agent_name'])
    monkeypatch.setenv('CODETETHER_PROVENANCE_SIGNING_KEYS', registry(value))
    proof = headers('claim', 'worker-1', name, 'cttask_1')
    key = verify(proof, 'claim', 'worker-1', name, 'cttask_1')
    assert key.agent_identity == name


def test_worker_proof_rejects_replay_for_another_task(monkeypatch):
    value = metadata()
    name = str(value['target_agent_name'])
    monkeypatch.setenv('CODETETHER_PROVENANCE_SIGNING_KEYS', registry(value))
    proof = headers('claim', 'worker-1', name, 'cttask_1')
    with pytest.raises(ValueError, match='invalid'):
        verify(proof, 'claim', 'worker-1', name, 'cttask_2')
''')
    write('test_worker_registration_identity.py', '''import pytest

from a2a_server.worker_registration_identity import bind
from tests.forgejo_metadata import metadata
from tests.forgejo_provenance_fixture import registry
from tests.worker_identity_headers import headers


def test_canonical_registration_is_bound_to_key_and_tenant(monkeypatch):
    value = metadata()
    name = str(value['target_agent_name'])
    monkeypatch.setenv('CODETETHER_PROVENANCE_SIGNING_KEYS', registry(value))
    proof = headers('register', 'worker-1', name, '')
    capabilities, tenant = bind(proof, 'worker-1', name, ['base'])
    assert tenant == 'tenant'
    assert 'codetether-identity-key:author-key' in capabilities


def test_canonical_registration_rejects_an_unbound_key(monkeypatch):
    value = metadata()
    monkeypatch.setenv('CODETETHER_PROVENANCE_SIGNING_KEYS', registry(value))
    proof = headers('register', 'worker-1', 'ctforgejo_attacker', '')
    with pytest.raises(ValueError, match='not bound'):
        bind(proof, 'worker-1', 'ctforgejo_attacker', [])
''')
    write('test_forgejo_worker_claim.py', '''import pytest

from a2a_server import database
from a2a_server.forgejo_worker_claim import require
from tests.forgejo_metadata import metadata
from tests.forgejo_provenance_fixture import registry
from tests.worker_identity_headers import headers


@pytest.mark.asyncio
async def test_author_task_claim_requires_the_bound_worker_key(monkeypatch):
    value = metadata()
    value.update(author_identity_key_id='author-key', tenant_id='tenant')
    name = str(value['target_agent_name'])

    async def task(_task_id):
        return {'metadata': value}

    async def worker(_worker_id):
        return {'name': name}

    monkeypatch.setenv('CODETETHER_PROVENANCE_SIGNING_KEYS', registry(value))
    monkeypatch.setattr(database, 'db_get_task', task)
    monkeypatch.setattr(database, 'db_get_worker', worker)
    proof = headers('claim', 'worker-1', name, 'cttask_1')
    await require(proof, 'cttask_1', 'worker-1')


@pytest.mark.asyncio
async def test_author_task_claim_rejects_another_worker(monkeypatch):
    value = metadata()
    value.update(author_identity_key_id='author-key', tenant_id='tenant')

    async def task(_task_id):
        return {'metadata': value}

    async def worker(_worker_id):
        return {'name': 'ctforgejo_attacker'}

    monkeypatch.setenv('CODETETHER_PROVENANCE_SIGNING_KEYS', registry(value))
    monkeypatch.setattr(database, 'db_get_task', task)
    monkeypatch.setattr(database, 'db_get_worker', worker)
    proof = headers('claim', 'worker-2', 'ctforgejo_attacker', 'cttask_1')
    with pytest.raises(ValueError):
        await require(proof, 'cttask_1', 'worker-2')
''')
    write('test_forgejo_worker_claim_endpoint.py', '''from types import SimpleNamespace

import pytest

from a2a_server import database, worker_sse
from tests.forgejo_metadata import metadata
from tests.forgejo_provenance_fixture import registry
from tests.worker_identity_headers import headers


@pytest.mark.asyncio
async def test_claim_endpoint_rejects_a_replayed_worker_proof(monkeypatch):
    value = metadata()
    value.update(author_identity_key_id='author-key', tenant_id='tenant')
    name = str(value['target_agent_name'])

    async def task(_task_id: str) -> dict[str, object]:
        return {'metadata': value}

    async def worker(_worker_id: str) -> dict[str, object]:
        return {'name': name}

    monkeypatch.setenv('CODETETHER_PROVENANCE_SIGNING_KEYS', registry(value))
    monkeypatch.setattr(database, 'db_get_task', task)
    monkeypatch.setattr(database, 'db_get_worker', worker)
    monkeypatch.setattr(worker_sse, '_verify_auth', lambda _request: None)
    request = SimpleNamespace(
        headers=headers('claim', 'worker-2', name, 'cttask_1')
    )
    claim = worker_sse.TaskClaimRequest(task_id='cttask_1')
    with pytest.raises(worker_sse.HTTPException) as raised:
        await worker_sse.claim_task(
            request, claim, worker_id=None, x_worker_id='worker-1'
        )
    assert raised.value.status_code == 403  # noqa: PLR2004
''')
    write('test_worker_registration_endpoint.py', '''import hashlib

import pytest

from a2a_server import monitor_api
from tests.forgejo_metadata import metadata
from tests.forgejo_provenance_fixture import registry
from tests.worker_identity_headers import headers
from tests.worker_signed_request import SignedRequest


@pytest.mark.asyncio
async def test_canonical_registration_persists_verified_tenant(monkeypatch):
    value = metadata()
    name = str(value['target_agent_name'])
    seen: dict[str, object] = {}

    async def persist(worker: dict[str, object]) -> bool:
        seen.update(worker)
        return True

    async def no_op(*_args: object, **_kwargs: object) -> None:
        return None

    monkeypatch.setenv('CODETETHER_PROVENANCE_SIGNING_KEYS', registry(value))
    monkeypatch.setattr(monitor_api.db, 'db_upsert_worker', persist)
    monkeypatch.setattr(monitor_api, '_redis_upsert_worker', no_op)
    monkeypatch.setattr(monitor_api.monitoring_service, 'log_message', no_op)
    registration = monitor_api.WorkerRegistration(worker_id='worker-1', name=name)
    body = b'{}'
    resource = f"worker-1:{hashlib.sha256(body).hexdigest()}"
    request = SignedRequest(headers('register', 'worker-1', name, resource), body)
    try:
        await monitor_api.register_worker(registration, request)
    finally:
        monitor_api._registered_workers.pop('worker-1', None)  # noqa: SLF001
    assert seen['tenant_id'] == 'tenant'
    assert 'codetether-identity-key:author-key' in seen['capabilities']
''')
    write('test_worker_registration_storage.py', '''import hashlib

import pytest

from a2a_server import monitor_api
from tests.forgejo_metadata import metadata
from tests.forgejo_provenance_fixture import registry
from tests.worker_identity_headers import headers
from tests.worker_signed_request import SignedRequest


@pytest.mark.asyncio
async def test_canonical_registration_fails_when_storage_does(monkeypatch):
    value = metadata()
    name = str(value['target_agent_name'])

    async def fail(_worker: dict[str, object]) -> bool:
        return False

    monkeypatch.setenv('CODETETHER_PROVENANCE_SIGNING_KEYS', registry(value))
    monkeypatch.setattr(monitor_api.db, 'db_upsert_worker', fail)
    registration = monitor_api.WorkerRegistration(worker_id='worker-2', name=name)
    body = b'{}'
    resource = f"worker-2:{hashlib.sha256(body).hexdigest()}"
    request = SignedRequest(headers('register', 'worker-2', name, resource), body)
    with pytest.raises(monitor_api.HTTPException) as raised:
        await monitor_api.register_worker(registration, request)
    assert raised.value.status_code == 503  # noqa: PLR2004
    assert 'worker-2' not in monitor_api._registered_workers  # noqa: SLF001
''')
    prepare = ROOT / 'test_forgejo_author_prepare.py'
    text = prepare.read_text()
    old = '''    value = metadata()
    install_database(monkeypatch, worker={'worker_id': 'worker-1'})
'''
    new = '''    value = metadata()
    value.update(author_identity_key_id='author-key', tenant_id='tenant')
    worker = {
        'worker_id': 'worker-1',
        'tenant_id': 'tenant',
        'capabilities': ['codetether-identity-key:author-key'],
    }
    install_database(monkeypatch, worker=worker)
'''
    if new not in text:
        if text.count(old) != 1:
            raise RuntimeError('verified worker prepare fixture anchor is missing')
        prepare.write_text(text.replace(old, new, 1))
    tenant_test = ROOT / 'test_forgejo_tenant_binding.py'
    tenant = tenant_test.read_text()
    tenant = tenant.replace(
        "        return {'worker_id': 'worker-1'}\n",
        '''        return {
            'worker_id': 'worker-1',
            'tenant_id': tenant_id,
            'capabilities': ['codetether-identity-key:author-key'],
        }
''',
    )
    tenant = tenant.replace(
        "    value['tenant_id'] = 'tenant-a'\n",
        "    value['tenant_id'] = 'tenant-a'\n    value['author_identity_key_id'] = 'author-key'\n",
    )
    tenant_test.write_text(tenant)


if __name__ == '__main__':
    main()