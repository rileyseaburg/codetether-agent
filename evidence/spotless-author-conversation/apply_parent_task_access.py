"""Bind Forgejo author-task reads and cancellation to their principal."""

from pathlib import Path

ROOT = Path('/home/riley/A2A-Server-MCP')
SERVER = ROOT / 'a2a_server'
TESTS = ROOT / 'tests'
MODULE_CONTENT = '''"""Principal authorization for persisted Forgejo author tasks."""

from collections.abc import Mapping

from fastapi import HTTPException, Request

from a2a_server.forgejo_provenance_keys import resolve as resolve_key
from a2a_server.forgejo_request_scope import resolve as resolve_scope
from a2a_server.forgejo_task_authorization import require

PROTOCOL = 'codetether.forgejo-author.v1'


def authorize(request: Request, task: Mapping[str, object]) -> None:
    """Require the original tenant or labeled bearer for a protocol task."""
    task_id = str(task.get('id') or '')
    metadata = task.get('metadata')
    if not isinstance(metadata, dict) or metadata.get('protocol') != PROTOCOL:
        if task_id.startswith('cttask_'):
            raise HTTPException(
                status_code=503, detail='Verified task binding is unavailable'
            )
        return
    try:
        key = resolve_key(str(metadata.get('author_identity_key_id') or ''))
        if metadata.get('tenant_id') != key.tenant_id:
            raise ValueError('task tenant does not match its provenance key')
        if metadata.get('target_agent_name') != key.agent_identity:
            raise ValueError('task identity does not match its provenance key')
        scope, tenant_id = resolve_scope(request)
        require(key, scope, tenant_id)
    except ValueError as error:
        raise HTTPException(status_code=403, detail=str(error)) from error
    except RuntimeError as error:
        raise HTTPException(status_code=503, detail=str(error)) from error
'''
ENDPOINT_TEST = '''from types import SimpleNamespace

import pytest

from a2a_server import monitor_api
from tests.forgejo_metadata import metadata
from tests.forgejo_provenance_fixture import registry


class Item:
    def __init__(self, value: dict[str, object]) -> None:
        self.value = value

    def to_dict(self) -> dict[str, object]:
        return self.value


class Bridge:
    def __init__(self, value: dict[str, object]) -> None:
        self.item = Item(value)
        self.cancelled = False

    async def get_task(self, _task_id: str) -> Item:
        return self.item

    def cancel_task(self, _task_id: str) -> bool:
        self.cancelled = True
        return True


@pytest.mark.asyncio
async def test_read_and_cancel_reject_an_unbound_principal(monkeypatch):
    value = metadata()
    value.update(author_identity_key_id='author-key', tenant_id='tenant')
    bridge = Bridge({'id': 'cttask_1', 'metadata': value})
    request = SimpleNamespace(
        headers={'authorization': 'Bearer other-token'},
        state=SimpleNamespace(),
    )
    monkeypatch.setenv('CODETETHER_PROVENANCE_SIGNING_KEYS', registry(value))
    monkeypatch.setenv('A2A_AUTH_TOKENS', 'other:other-token')
    monkeypatch.setattr(monitor_api, 'get_agent_bridge', lambda: bridge)
    with pytest.raises(monitor_api.HTTPException):
        await monitor_api.get_task('cttask_1', request)
    with pytest.raises(monitor_api.HTTPException):
        await monitor_api.cancel_task('cttask_1', request)
    with pytest.raises(monitor_api.HTTPException):
        await monitor_api.get_task_output('cttask_1', request)
    with pytest.raises(monitor_api.HTTPException):
        await monitor_api.stream_task_output_sse('cttask_1', request)
    assert not bridge.cancelled
'''
TEST_CONTENT = '''from types import SimpleNamespace

import pytest
from fastapi import HTTPException

from a2a_server.forgejo_task_access import authorize
from tests.forgejo_metadata import metadata
from tests.forgejo_provenance_fixture import registry


def request(token: str) -> SimpleNamespace:
    return SimpleNamespace(
        headers={'authorization': f'Bearer {token}'},
        state=SimpleNamespace(),
    )


def task() -> dict[str, object]:
    value = metadata()
    value.update(
        author_identity_key_id='author-key',
        tenant_id='tenant',
    )
    return {'id': 'cttask_1', 'metadata': value}


def test_task_access_requires_the_bound_bearer_label(monkeypatch):
    value = metadata()
    monkeypatch.setenv('CODETETHER_PROVENANCE_SIGNING_KEYS', registry(value))
    monkeypatch.setenv(
        'A2A_AUTH_TOKENS', 'reviewer:task-token,other:other-token'
    )
    authorize(request('task-token'), task())
    with pytest.raises(HTTPException) as raised:
        authorize(request('other-token'), task())
    assert raised.value.status_code == 403  # noqa: PLR2004


def test_reserved_task_without_a_binding_fails_closed():
    with pytest.raises(HTTPException) as raised:
        authorize(request('task-token'), {'id': 'cttask_1', 'metadata': {}})
    assert raised.value.status_code == 503  # noqa: PLR2004
'''


def main() -> None:
    """Install task-access authorization and focused regression tests."""
    (SERVER / 'forgejo_task_access.py').write_text(MODULE_CONTENT)
    monitor = SERVER / 'monitor_api.py'
    text = monitor.read_text()
    import_line = 'from .forgejo_task_access import authorize as authorize_forgejo_task_access\n'
    if import_line not in text:
        anchor = 'from .forgejo_task_response import public as public_forgejo_task\n'
        text = text.replace(anchor, anchor + import_line, 1)
    text = text.replace(
        '''async def list_workspace_tasks(workspace_id: str, status: Optional[str] = None):
''',
        '''async def list_workspace_tasks(
    workspace_id: str, request: Request, status: Optional[str] = None
):
''',
        1,
    )
    list_return = '    return tasks\n\n\n@agent_router_alias.get(\'/tasks/{task_id}\''
    list_guard = '''    visible = []
    for task in tasks:
        try:
            authorize_forgejo_task_access(request, task)
        except HTTPException as error:
            if error.status_code in (401, 403):
                continue
            raise
        visible.append(public_forgejo_task(task))
    return visible


@agent_router_alias.get('/tasks/{task_id}'
'''
    if list_guard not in text:
        text = text.replace(list_return, list_guard, 1)
    text = text.replace(
        "@agent_router_alias.get('/tasks/{task_id}'\n, response_model",
        "@agent_router_alias.get('/tasks/{task_id}', response_model",
        1,
    )
    text = text.replace(
        'async def get_task(task_id: str):\n',
        'async def get_task(task_id: str, request: Request):\n',
        1,
    )
    get_return = '    return public_forgejo_task(task.to_dict())\n'
    get_guard = '''    task_data = task.to_dict()
    authorize_forgejo_task_access(request, task_data)
    return public_forgejo_task(task_data)
'''
    if get_guard not in text:
        text = text.replace(get_return, get_guard, 1)
    text = text.replace(
        'async def cancel_task(task_id: str):\n',
        'async def cancel_task(task_id: str, request: Request):\n',
    )
    cancel_anchor = '''    success = bridge.cancel_task(task_id)
'''
    cancel_guard = '''    task = await bridge.get_task(task_id)
    if not task:
        raise HTTPException(status_code=404, detail='Task not found')
    authorize_forgejo_task_access(request, task.to_dict())
    success = bridge.cancel_task(task_id)
'''
    text = text.replace(cancel_anchor, cancel_guard)
    text = text.replace(
        '''async def get_task_output(task_id: str, since: Optional[int] = None):
''',
        '''async def get_task_output(
    task_id: str, request: Request, since: Optional[int] = None
):
''',
        1,
    )
    output_doc = '    """Get streaming output for a task."""\n'
    output_gate = output_doc + '''    bridge = get_agent_bridge()
    if bridge is None:
        raise HTTPException(status_code=503, detail='Agent bridge not available')
    task = await bridge.get_task(task_id)
    if not task:
        raise HTTPException(status_code=404, detail='Task not found')
    authorize_forgejo_task_access(request, task.to_dict())
'''
    if output_gate not in text:
        text = text.replace(output_doc, output_gate, 1)
    stream_doc = '    """SSE stream for real-time task output."""\n'
    stream_gate = stream_doc + '''    bridge = get_agent_bridge()
    if bridge is None:
        raise HTTPException(status_code=503, detail='Agent bridge not available')
    task = await bridge.get_task(task_id)
    if not task:
        raise HTTPException(status_code=404, detail='Task not found')
    authorize_forgejo_task_access(request, task.to_dict())
'''
    if stream_gate not in text:
        text = text.replace(stream_doc, stream_gate, 1)
    monitor.write_text(text)
    (TESTS / 'test_forgejo_task_access.py').write_text(TEST_CONTENT)
    (TESTS / 'test_forgejo_task_access_endpoint.py').write_text(ENDPOINT_TEST)


if __name__ == '__main__':
    main()