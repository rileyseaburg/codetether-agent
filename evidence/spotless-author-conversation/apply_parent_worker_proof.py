"""Install cryptographic ownership proof for canonical worker identities."""

from pathlib import Path

ROOT = Path('/home/riley/A2A-Server-MCP')
SERVER = ROOT / 'a2a_server'


def write(name: str, content: str) -> None:
    """Write one focused server module."""
    (SERVER / name).write_text(content)


def main() -> None:
    """Create proof verification, registration, claim, and lookup bindings."""
    write('worker_identity_proof_payload.py', '''"""Canonical payload for worker identity possession proofs."""


def canonical(
    action: str, worker_id: str, name: str, resource: str, timestamp: str
) -> bytes:
    """Encode one domain-separated worker proof payload."""
    values = ('codetether-worker-proof-v1', action, worker_id, name, resource, timestamp)
    return '\\n'.join(values).encode()
''')
    write('worker_identity_proof.py', '''"""Verification of per-request canonical worker identity proofs."""

import hashlib
import hmac
import time

from collections.abc import Mapping

from a2a_server.forgejo_provenance_keys import ProvenanceKey, resolve
from a2a_server.worker_identity_proof_payload import canonical

MAX_CLOCK_SKEW_SECONDS = 120
SIGNATURE_LENGTH = 64


def verify(
    headers: Mapping[str, str], action: str, worker_id: str, name: str, resource: str
) -> ProvenanceKey:
    """Verify a fresh HMAC proof bound to the request and canonical identity."""
    key_id = headers.get('x-codetether-key-id', '')
    timestamp = headers.get('x-codetether-proof-timestamp', '')
    signature = headers.get('x-codetether-worker-proof', '')
    try:
        age = abs(int(time.time()) - int(timestamp))
    except ValueError as error:
        raise ValueError('worker identity proof timestamp is invalid') from error
    if age > MAX_CLOCK_SKEW_SECONDS:
        raise ValueError('worker identity proof is stale')
    key = resolve(key_id)
    if key.agent_identity != name:
        raise ValueError('worker proof key is not bound to the agent identity')
    expected = hmac.new(
        key.secret.encode(), canonical(action, worker_id, name, resource, timestamp), hashlib.sha256
    ).hexdigest()
    if len(signature) != SIGNATURE_LENGTH or not hmac.compare_digest(signature, expected):
        raise ValueError('worker identity proof is invalid')
    return key
''')
    write('worker_registration_identity.py', '''"""Trusted identity fields for canonical worker registration."""

from collections.abc import Mapping, Sequence

from a2a_server.worker_identity_proof import verify

PREFIX = 'ctforgejo_'


def bind(
    headers: Mapping[str, str], worker_id: str, name: str, capabilities: Sequence[str]
) -> tuple[list[str], str | None]:
    """Return trusted tenancy after canonical-route key possession."""
    values = list(capabilities)
    if not name.startswith(PREFIX):
        return values, None
    key = verify(headers, 'register', worker_id, name, '')
    marker = f'codetether-identity-key:{key.key_id}'
    if marker not in values:
        values.append(marker)
    return values, key.tenant_id
''')
    write('forgejo_worker_binding.py', '''"""Durable worker checks for a verified author task."""

from collections.abc import Mapping


def require(worker: Mapping[str, object], metadata: Mapping[str, object]) -> None:
    """Require the selected worker to own the task key and tenant."""
    key_id = str(metadata.get('author_identity_key_id') or '')
    marker = f'codetether-identity-key:{key_id}'
    capabilities = worker.get('capabilities')
    if not isinstance(capabilities, list) or marker not in capabilities:
        raise LookupError('canonical author worker identity is not verified')
    tenant = str(metadata.get('tenant_id') or '')
    if str(worker.get('tenant_id') or '') != tenant:
        raise LookupError('canonical author worker tenant does not match')
''')
    write('forgejo_worker_claim.py', '''"""Identity proof gate for claiming verified Forgejo author tasks."""

from collections.abc import Mapping

from a2a_server import database as db
from a2a_server.worker_identity_proof import verify

PROTOCOL = 'codetether.forgejo-author.v1'


async def require(headers: Mapping[str, str], task_id: str, worker_id: str) -> None:
    """Require the target worker key before releasing a verified author task."""
    task = await db.db_get_task(task_id)
    if not task:
        if task_id.startswith('cttask_'):
            raise LookupError('verified task does not exist in durable storage')
        return
    metadata = task.get('metadata')
    if not isinstance(metadata, dict) or metadata.get('protocol') != PROTOCOL:
        return
    worker = await db.db_get_worker(worker_id)
    if not worker:
        raise LookupError('claiming worker is not durably registered')
    name = str(worker.get('name') or '')
    key = verify(headers, 'claim', worker_id, name, task_id)
    if metadata.get('author_identity_key_id') != key.key_id:
        raise ValueError('worker proof key does not match the author task')
    if metadata.get('target_agent_name') != key.agent_identity:
        raise ValueError('worker proof identity does not match the author task')
    if metadata.get('tenant_id') != key.tenant_id:
        raise ValueError('worker proof tenant does not match the author task')
''')
    monitor = SERVER / 'monitor_api.py'
    text = monitor.read_text()
    text = text.replace(
        "            'status': 'active',\n"
        "        'tenant_id': tenant_id,\n"
        "            'messages_count': 0,\n",
        "            'status': 'active',\n"
        "            'messages_count': 0,\n",
    )
    registration_import = (
        'from .worker_registration_identity import bind as bind_worker_identity\n'
    )
    if registration_import not in text:
        anchor = 'from . import database as db\n'
        text = text.replace(anchor, anchor + registration_import, 1)
    text = text.replace(
        'async def register_worker(registration: WorkerRegistration):\n',
        'async def register_worker(registration: WorkerRegistration, request: Request):\n',
        1,
    )
    old_caps = '''    worker_info = {
        'worker_id': registration.worker_id,
        'name': registration.name,
        'capabilities': _normalized_worker_capabilities(
            registration.name,
            registration.capabilities,
        ),
'''
    new_caps = '''    capabilities = _normalized_worker_capabilities(
        registration.name, registration.capabilities
    )
    try:
        capabilities, tenant_id = bind_worker_identity(
            request.headers, registration.worker_id, registration.name, capabilities
        )
    except ValueError as error:
        raise HTTPException(status_code=403, detail=str(error)) from error
    except RuntimeError as error:
        raise HTTPException(status_code=503, detail=str(error)) from error
    worker_info = {
        'worker_id': registration.worker_id,
        'name': registration.name,
        'capabilities': capabilities,
'''
    if new_caps not in text:
        if text.count(old_caps) != 1:
            raise RuntimeError('worker registration binding anchor is missing')
        text = text.replace(old_caps, new_caps, 1)
    worker_end = '''        'status': 'active',
    }

    # In-memory cache for this instance
'''
    worker_tenant_end = '''        'status': 'active',
        'tenant_id': tenant_id,
    }

    # In-memory cache for this instance
'''
    if worker_tenant_end not in text:
        text = text.replace(worker_end, worker_tenant_end, 1)
    persist_old = '    await db.db_upsert_worker(worker_info)\n'
    persist_new = '''    persisted = await db.db_upsert_worker(worker_info)
    if registration.name.startswith('ctforgejo_') and not persisted:
        _registered_workers.pop(registration.worker_id, None)
        raise HTTPException(
            status_code=503,
            detail='canonical worker registration was not durably persisted',
        )
'''
    if persist_new not in text:
        text = text.replace(persist_old, persist_new, 1)
    monitor.write_text(text)
    task = SERVER / 'forgejo_author_task.py'
    task_text = task.read_text()
    binding_import = 'from a2a_server.forgejo_worker_binding import require as require_worker_binding\n'
    if binding_import not in task_text:
        anchor = 'from a2a_server.forgejo_author_contract import validate\n'
        task_text = task_text.replace(anchor, anchor + binding_import, 1)
    worker_anchor = "    if not worker:\n        raise LookupError('canonical author worker is not active')\n"
    binding_call = worker_anchor + '    require_worker_binding(worker, metadata)\n'
    if binding_call not in task_text:
        if task_text.count(worker_anchor) != 1:
            raise RuntimeError('durable worker binding anchor is missing')
        task_text = task_text.replace(worker_anchor, binding_call, 1)
    task.write_text(task_text)
    worker_sse = SERVER / 'worker_sse.py'
    sse = worker_sse.read_text()
    import_line = 'from .forgejo_worker_claim import require as require_forgejo_worker_claim\n'
    if import_line not in sse:
        anchor = 'from .sequencer_store import SequencerStore\n'
        sse = sse.replace(anchor, anchor + import_line, 1)
    claim_anchor = '''    registry = get_worker_registry()

    success = await registry.claim_task(claim.task_id, resolved_worker_id)
'''
    claim_gate = '''    try:
        await require_forgejo_worker_claim(
            request.headers, claim.task_id, resolved_worker_id
        )
    except ValueError as error:
        raise HTTPException(status_code=403, detail=str(error)) from error
    except (LookupError, RuntimeError) as error:
        raise HTTPException(status_code=503, detail=str(error)) from error

    registry = get_worker_registry()

    success = await registry.claim_task(claim.task_id, resolved_worker_id)
'''
    if claim_gate not in sse:
        if sse.count(claim_anchor) != 1:
            raise RuntimeError('worker claim proof anchor is missing')
        sse = sse.replace(claim_anchor, claim_gate, 1)
    worker_sse.write_text(sse)


if __name__ == '__main__':
    main()