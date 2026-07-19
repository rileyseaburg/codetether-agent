"""Require canonical worker proof for all author-task mutations."""

from pathlib import Path

ROOT = Path('/home/riley/A2A-Server-MCP')
SERVER = ROOT / 'a2a_server'
TESTS = ROOT / 'tests'
HTTP_CONTENT = '''"""HTTP error mapping for worker task-mutation identity proofs."""

from fastapi import HTTPException, Request

from a2a_server.forgejo_worker_claim import require
from a2a_server.worker_request_resource import derive


async def authorize(
    request: Request, action: str, task_id: str, worker_id: str
) -> None:
    """Authorize a task mutation or raise its typed HTTP failure."""
    resource = await derive(request, task_id)
    try:
        await require(
            request.headers, task_id, worker_id, action=action, resource=resource
        )
    except ValueError as error:
        raise HTTPException(status_code=403, detail=str(error)) from error
    except (LookupError, RuntimeError) as error:
        raise HTTPException(status_code=503, detail=str(error)) from error
'''
RESOURCE_CONTENT = '''"""Exact request-body resource binding for worker proofs."""

import hashlib

from fastapi import Request


async def derive(request: Request, subject: str) -> str:
    """Bind a worker operation to its subject and exact request bytes."""
    body_hash = hashlib.sha256(await request.body()).hexdigest()
    return f'{subject}:{body_hash}'
'''
OWNERSHIP_CONTENT = '''"""Durable claim ownership policy for author-task mutations."""

from collections.abc import Mapping


def require(
    task: Mapping[str, object], worker_id: str, action: str
) -> None:
    """Require the claiming worker and an active task for every mutation."""
    if action == 'claim':
        return
    if str(task.get('worker_id') or '') != worker_id:
        raise ValueError('worker does not own the author task claim')
    if str(task.get('status') or '') not in {'running', 'working'}:
        raise ValueError('author task is not active')
'''
REQUEST_FIXTURE = '''"""Minimal request carrying signed headers and raw JSON bytes."""


class SignedRequest:
    """Request-like test value consumed by mutation authorization."""

    def __init__(self, headers: dict[str, str], body: bytes = b'{}') -> None:
        self.headers = headers
        self._body = body

    async def body(self) -> bytes:
        return self._body
'''
UNIT_TEST = '''import hashlib
import pytest
from fastapi import HTTPException

from a2a_server import database
from a2a_server.forgejo_worker_claim import require
from a2a_server.worker_task_mutation import authorize
from tests.forgejo_metadata import metadata
from tests.forgejo_provenance_fixture import registry
from tests.worker_identity_headers import headers
from tests.worker_signed_request import SignedRequest


@pytest.mark.asyncio
async def test_task_mutation_proof_is_bound_to_its_action(monkeypatch):
    value = metadata()
    value.update(author_identity_key_id='author-key', tenant_id='tenant')
    name = str(value['target_agent_name'])

    async def task(_task_id: str) -> dict[str, object]:
        return {'metadata': value, 'worker_id': 'worker-1', 'status': 'working'}

    async def worker(_worker_id: str) -> dict[str, object]:
        return {'name': name}

    monkeypatch.setenv('CODETETHER_PROVENANCE_SIGNING_KEYS', registry(value))
    monkeypatch.setattr(database, 'db_get_task', task)
    monkeypatch.setattr(database, 'db_get_worker', worker)
    proof = headers('release', 'worker-1', name, 'cttask_1')
    await require(proof, 'cttask_1', 'worker-1', action='release')
    with pytest.raises(ValueError):
        await require(proof, 'cttask_1', 'worker-1', action='output')

    body = b'{"task_id":"cttask_1","status":"completed"}'
    resource = f"cttask_1:{hashlib.sha256(body).hexdigest()}"
    release_proof = headers('release', 'worker-1', name, resource)
    await authorize(
        SignedRequest(release_proof, body),
        'release',
        'cttask_1',
        'worker-1',
    )
    with pytest.raises(HTTPException):
        await authorize(
            SignedRequest(release_proof, body + b' '),
            'release',
            'cttask_1',
            'worker-1',
        )
    with pytest.raises(ValueError):
        await require(
            release_proof, 'cttask_1', 'worker-2', action='release'
        )
'''
ENDPOINT_TEST = '''import pytest

from a2a_server import database, worker_sse
from tests.forgejo_metadata import metadata
from tests.forgejo_provenance_fixture import registry
from tests.worker_identity_headers import headers
from tests.worker_signed_request import SignedRequest


@pytest.mark.asyncio
async def test_release_endpoint_rejects_a_claim_proof_replay(monkeypatch):
    value = metadata()
    value.update(author_identity_key_id='author-key', tenant_id='tenant')
    name = str(value['target_agent_name'])

    async def task(_task_id: str) -> dict[str, object]:
        return {'metadata': value, 'worker_id': 'worker-1', 'status': 'working'}

    async def worker(_worker_id: str) -> dict[str, object]:
        return {'name': name}

    monkeypatch.setenv('CODETETHER_PROVENANCE_SIGNING_KEYS', registry(value))
    monkeypatch.setattr(database, 'db_get_task', task)
    monkeypatch.setattr(database, 'db_get_worker', worker)
    monkeypatch.setattr(worker_sse, '_verify_auth', lambda _request: None)
    request = SignedRequest(
        headers('claim', 'worker-1', name, 'cttask_1')
    )
    release = worker_sse.TaskReleaseRequest(
        task_id='cttask_1', status='completed'
    )
    with pytest.raises(worker_sse.HTTPException) as raised:
        await worker_sse.release_task(
            request, release, worker_id=None, x_worker_id='worker-1'
        )
    assert raised.value.status_code == 403  # noqa: PLR2004
'''
EXTENDED_TEST = '''import pytest

from a2a_server import database, worker_progress_routes
from tests.forgejo_metadata import metadata
from tests.forgejo_provenance_fixture import registry
from tests.worker_identity_headers import headers
from tests.worker_signed_request import SignedRequest


@pytest.mark.asyncio
async def test_extended_heartbeat_rejects_a_claim_proof_replay(monkeypatch):
    value = metadata()
    value.update(author_identity_key_id='author-key', tenant_id='tenant')
    name = str(value['target_agent_name'])

    async def task(_task_id: str) -> dict[str, object]:
        return {'metadata': value, 'worker_id': 'worker-1', 'status': 'working'}

    async def worker(_worker_id: str) -> dict[str, object]:
        return {'name': name}

    monkeypatch.setenv('CODETETHER_PROVENANCE_SIGNING_KEYS', registry(value))
    monkeypatch.setattr(database, 'db_get_task', task)
    monkeypatch.setattr(database, 'db_get_worker', worker)
    monkeypatch.setattr(worker_progress_routes, 'verify_auth', lambda _request: None)
    request = SignedRequest(
        headers('claim', 'worker-1', name, 'cttask_1')
    )
    heartbeat = worker_progress_routes.ExtendedHeartbeatRequest(
        task_id='cttask_1', worker_id='worker-1'
    )
    with pytest.raises(worker_progress_routes.HTTPException) as raised:
        await worker_progress_routes.heartbeat_extended_endpoint(
            request, heartbeat
        )
    assert raised.value.status_code == 403  # noqa: PLR2004
    with pytest.raises(worker_progress_routes.HTTPException):
        await worker_progress_routes.post_extended_heartbeat(request, heartbeat)
    regular = worker_progress_routes.TaskHeartbeatRequest(
        task_id='cttask_1', worker_id='worker-1'
    )
    with pytest.raises(worker_progress_routes.HTTPException):
        await worker_progress_routes.post_task_heartbeat(request, regular)
    resume = worker_progress_routes.TaskResumeRequest(
        task_id='cttask_1', worker_id='worker-1'
    )
    with pytest.raises(worker_progress_routes.HTTPException):
        await worker_progress_routes.resume_task_from_checkpoint(
            request, resume
        )
'''
MONITOR_TEST = '''import pytest

from a2a_server import database, monitor_api
from tests.forgejo_metadata import metadata
from tests.forgejo_provenance_fixture import registry
from tests.worker_identity_headers import headers
from tests.worker_signed_request import SignedRequest


def install(monkeypatch):
    value = metadata()
    value.update(author_identity_key_id='author-key', tenant_id='tenant')
    name = str(value['target_agent_name'])

    async def task(_task_id: str) -> dict[str, object]:
        return {'metadata': value, 'worker_id': 'worker-1', 'status': 'working'}

    async def worker(_worker_id: str) -> dict[str, object]:
        return {'name': name}

    monkeypatch.setenv('CODETETHER_PROVENANCE_SIGNING_KEYS', registry(value))
    monkeypatch.setattr(database, 'db_get_task', task)
    monkeypatch.setattr(database, 'db_get_worker', worker)
    return SignedRequest(
        headers('claim', 'worker-1', name, 'cttask_1')
    )


@pytest.mark.asyncio
async def test_status_endpoint_rejects_a_claim_proof_replay(monkeypatch):
    request = install(monkeypatch)
    update = monitor_api.TaskStatusUpdate(status='completed', worker_id='worker-1')
    with pytest.raises(monitor_api.HTTPException) as raised:
        await monitor_api.update_task_status('cttask_1', update, request)
    assert raised.value.status_code == 403  # noqa: PLR2004


@pytest.mark.asyncio
async def test_output_endpoint_rejects_a_claim_proof_replay(monkeypatch):
    request = install(monkeypatch)
    chunk = monitor_api.TaskOutputChunk(worker_id='worker-1', output='forged')
    with pytest.raises(monitor_api.HTTPException) as raised:
        await monitor_api.stream_task_output('cttask_1', chunk, request)
    assert raised.value.status_code == 403  # noqa: PLR2004
'''


def main() -> None:
    """Install action-bound mutation proof gates and focused tests."""
    (SERVER / 'forgejo_worker_ownership.py').write_text(
        OWNERSHIP_CONTENT
    )
    claim = SERVER / 'forgejo_worker_claim.py'
    text = claim.read_text()
    ownership_import = 'from a2a_server.forgejo_worker_ownership import require as require_ownership\n'
    if ownership_import not in text:
        anchor = 'from a2a_server.worker_identity_proof import verify\n'
        text = text.replace(anchor, anchor + ownership_import, 1)
    old_signature = '''async def require(
    headers: Mapping[str, str], task_id: str, worker_id: str
) -> None:
'''
    action_signature = '''async def require(
    headers: Mapping[str, str],
    task_id: str,
    worker_id: str,
    *,
    action: str = 'claim',
) -> None:
'''
    new_signature = '''async def require(
    headers: Mapping[str, str],
    task_id: str,
    worker_id: str,
    *,
    action: str = 'claim',
    resource: str | None = None,
) -> None:
'''
    if new_signature not in text:
        if action_signature in text:
            text = text.replace(action_signature, new_signature, 1)
        else:
            text = text.replace(old_signature, new_signature, 1)
    text = text.replace(
        "verify(headers, 'claim', worker_id, name, task_id)",
        'verify(headers, action, worker_id, name, resource or task_id)',
        1,
    )
    text = text.replace(
        'verify(headers, action, worker_id, name, task_id)',
        'verify(headers, action, worker_id, name, resource or task_id)',
        1,
    )
    protocol_gate = '''    if not isinstance(metadata, dict) or metadata.get('protocol') != PROTOCOL:
        return
'''
    ownership_gate = protocol_gate + '''    require_ownership(task, worker_id, action)
'''
    if ownership_gate not in text:
        text = text.replace(protocol_gate, ownership_gate, 1)
    claim.write_text(text)
    (SERVER / 'worker_task_mutation.py').write_text(HTTP_CONTENT)
    (SERVER / 'worker_request_resource.py').write_text(RESOURCE_CONTENT)
    sse = SERVER / 'worker_sse.py'
    text = sse.read_text()
    import_line = 'from .worker_task_mutation import authorize as authorize_worker_mutation\n'
    if import_line not in text:
        anchor = 'from .forgejo_worker_claim import require as require_forgejo_worker_claim\n'
        text = text.replace(anchor, anchor + import_line, 1)
    release_anchor = '''    resolved_worker_id = worker_id or x_worker_id
    if not resolved_worker_id:
        raise HTTPException(
            status_code=400,
            detail='worker_id is required (query param or X-Worker-ID header)',
        )

    registry = get_worker_registry()
'''
    release_gate = release_anchor.replace(
        '    registry = get_worker_registry()\n',
        '''    await authorize_worker_mutation(
        request, 'release', release.task_id, resolved_worker_id
    )
    registry = get_worker_registry()
''',
    )
    release_section = text.index("@worker_sse_router.post('/tasks/release')")
    prefix, suffix = text[:release_section], text[release_section:]
    suffix = suffix.replace(
        "request.headers, 'release'", "request, 'release'", 1
    )
    if "request.headers, 'release'" not in suffix:
        suffix = suffix.replace(release_anchor, release_gate, 1)
    sse.write_text(prefix + suffix)
    monitor = SERVER / 'monitor_api.py'
    text = monitor.read_text()
    if import_line not in text:
        anchor = 'from . import database as db\n'
        text = text.replace(anchor, anchor + import_line, 1)
    text = text.replace(
        'async def update_task_status(task_id: str, update: TaskStatusUpdate):\n',
        'async def update_task_status(task_id: str, update: TaskStatusUpdate, request: Request):\n',
        1,
    )
    status_doc = '    """Update task status (called by workers)."""\n'
    status_gate = status_doc + '''    await authorize_worker_mutation(
        request, 'status', task_id, update.worker_id
    )
'''
    if status_gate not in text:
        text = text.replace(status_doc, status_gate, 1)
    status_call = status_gate.removeprefix(status_doc)
    text = text.replace(status_gate + status_call, status_gate, 1)
    text = text.replace(
        "request.headers, 'status'", "request, 'status'", 1
    )
    text = text.replace(
        'async def stream_task_output(task_id: str, chunk: TaskOutputChunk):\n',
        'async def stream_task_output(task_id: str, chunk: TaskOutputChunk, request: Request):\n',
        1,
    )
    output_doc = '    """Receive streaming output from a worker (called by workers)."""\n'
    output_gate = output_doc + '''    await authorize_worker_mutation(
        request, 'output', task_id, chunk.worker_id
    )
'''
    if output_gate not in text:
        text = text.replace(output_doc, output_gate, 1)
    output_call = output_gate.removeprefix(output_doc)
    text = text.replace(output_gate + output_call, output_gate, 1)
    text = text.replace(
        "request.headers, 'output'", "request, 'output'", 1
    )
    monitor.write_text(text)
    progress = SERVER / 'worker_progress_routes.py'
    text = progress.read_text()
    if import_line not in text:
        anchor = 'from .worker_auth import verify_auth\n'
        text = text.replace(anchor, anchor + import_line, 1)
    heartbeat_doc = '''    """Legacy extended heartbeat endpoint for persistent workers."""
    verify_auth(request)
'''
    heartbeat_gate = heartbeat_doc + '''    await authorize_worker_mutation(
        request, 'heartbeat-extended', heartbeat.task_id, heartbeat.worker_id
    )
'''
    if heartbeat_gate not in text:
        text = text.replace(heartbeat_doc, heartbeat_gate, 1)
    heartbeat_call = heartbeat_gate.removeprefix(heartbeat_doc)
    text = text.replace(heartbeat_gate + heartbeat_call, heartbeat_gate, 1)
    text = text.replace(
        "request.headers, 'heartbeat-extended'", "request, 'heartbeat-extended'", 1
    )
    regular_doc = '''    """Persist CI runner progress and renew its short lease."""
    verify_auth(request)
'''
    regular_gate = regular_doc + '''    await authorize_worker_mutation(
        request, 'heartbeat-progress', heartbeat.task_id, heartbeat.worker_id
    )
'''
    if regular_gate not in text:
        text = text.replace(regular_doc, regular_gate, 1)
    extended_doc = '''    """Extended heartbeat for 7-day persistent worker tasks."""
    verify_auth(request)
'''
    extended_gate = extended_doc + '''    await authorize_worker_mutation(
        request, 'heartbeat-extended', heartbeat.task_id, heartbeat.worker_id
    )
'''
    if extended_gate not in text:
        text = text.replace(extended_doc, extended_gate, 1)
    resume_doc = '''    """Resume a task from its last checkpoint."""
    verify_auth(request)
'''
    resume_gate = resume_doc + '''    await authorize_worker_mutation(
        request, 'resume', resume.task_id, resume.worker_id
    )
'''
    if resume_gate not in text:
        text = text.replace(resume_doc, resume_gate, 1)
    progress.write_text(text)
    (TESTS / 'test_worker_mutation_proof.py').write_text(UNIT_TEST)
    (TESTS / 'worker_signed_request.py').write_text(REQUEST_FIXTURE)
    (TESTS / 'test_worker_release_proof.py').write_text(ENDPOINT_TEST)
    (TESTS / 'test_worker_monitor_mutation_proof.py').write_text(MONITOR_TEST)
    (TESTS / 'test_worker_extended_heartbeat_proof.py').write_text(
        EXTENDED_TEST
    )


if __name__ == '__main__':
    main()