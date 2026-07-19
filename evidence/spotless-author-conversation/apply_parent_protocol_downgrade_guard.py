"""Reject anonymous session-resume claims outside the verified protocol."""

from pathlib import Path

ROOT = Path('/home/riley/A2A-Server-MCP')
GUARD = ROOT / 'a2a_server/forgejo_protocol_guard.py'
MONITOR = ROOT / 'a2a_server/monitor_api.py'
TEST = ROOT / 'tests/test_forgejo_protocol_downgrade.py'
GUARD_CONTENT = '''"""Fail-closed classification of Forgejo author task envelopes."""

from collections.abc import Mapping

PROTOCOL = 'codetether.forgejo-author.v1'
PROTECTED_FIELDS = (
    'resume_session_id',
    'author_provenance_id',
    'author_agent_identity',
    'provenance_verified',
    'preserve_session_workspace',
    'server_author_binding_verified',
    'author_identity_key_id',
)


def classify(metadata: Mapping[str, object]) -> bool:
    """Return true for the exact protocol or reject protected downgrade data."""
    protocol = str(metadata.get('protocol') or '')
    if protocol == PROTOCOL:
        return True
    protected = any(metadata.get(field) is not None for field in PROTECTED_FIELDS)
    forgejo_alias = protocol.startswith('codetether.forgejo-author')
    if protected or forgejo_alias:
        raise ValueError('verified author metadata requires the exact protocol')
    return False
'''


def main() -> None:
    """Write the guard, invoke it at HTTP ingress, and add downgrade coverage."""
    GUARD.write_text(GUARD_CONTENT)
    text = MONITOR.read_text()
    import_line = 'from .forgejo_protocol_guard import classify as classify_forgejo_protocol\n'
    if import_line not in text:
        anchor = 'from .forgejo_request_scope import resolve as forgejo_request_scope\n'
        text = text.replace(anchor, anchor + import_line, 1)
    text = text.replace(
        'forgejo_request_scope.resolve(request)', 'forgejo_request_scope(request)'
    )
    old = '''    scope, tenant_id = 'internal:global', None
    metadata = task_data.metadata or {}
    if metadata.get('protocol') == 'codetether.forgejo-author.v1':
        scope, tenant_id = forgejo_request_scope(request)
'''
    new = '''    metadata = task_data.metadata or {}
    try:
        is_forgejo_protocol = classify_forgejo_protocol(metadata)
    except ValueError as error:
        raise HTTPException(status_code=422, detail=str(error)) from error
    if is_forgejo_protocol:
        scope, tenant_id = forgejo_request_scope(request)
    else:
        scope, tenant_id = 'internal:global', None
'''
    if new not in text:
        if text.count(old) != 1:
            raise RuntimeError('protocol ingress guard anchor is missing')
        text = text.replace(old, new, 1)
    core_anchor = '''    base_metadata = task_data.metadata.copy() if task_data.metadata else {}
'''
    core_guard = '''    base_metadata = task_data.metadata.copy() if task_data.metadata else {}
    try:
        classify_forgejo_protocol(base_metadata)
    except ValueError as error:
        raise HTTPException(status_code=422, detail=str(error)) from error
'''
    if core_guard not in text:
        if core_anchor not in text:
            raise RuntimeError('core protocol guard anchor is missing')
        text = text.replace(core_anchor, core_guard)
    MONITOR.write_text(text)
    test = '''from http import HTTPStatus
from types import SimpleNamespace

import pytest

import a2a_server.monitor_api as monitor


@pytest.mark.asyncio
async def test_protocol_downgrade_cannot_reuse_an_author_session(monkeypatch):
    task = monitor.AgentTaskCreate(
        title='attack',
        prompt='data',
        metadata={
            'resume_session_id': 'victim-session',
            'provenance_verified': True,
            'preserve_session_workspace': True,
        },
    )
    request = SimpleNamespace(headers={})

    async def unexpected_create(*_arguments):
        raise AssertionError('downgraded task must not be created')

    monkeypatch.setattr(monitor, 'create_global_task', unexpected_create)
    with pytest.raises(monitor.HTTPException) as error:
        await monitor.create_global_task_endpoint(task, request)
    assert error.value.status_code == HTTPStatus.UNPROCESSABLE_ENTITY
'''
    TEST.write_text(test)


if __name__ == '__main__':
    main()