"""Replace a wide service call with one cohesive author request value."""

from pathlib import Path

ROOT = Path("/home/riley/A2A-Server-MCP")
REQUEST = ROOT / "a2a_server/forgejo_author_request.py"
SERVICE = ROOT / "a2a_server/forgejo_author_service.py"
MONITOR = ROOT / "a2a_server/monitor_api.py"
FIXTURE = ROOT / "tests/forgejo_service_fixture.py"
SERVICE_TEST = ROOT / "tests/test_forgejo_author_service.py"
CONCURRENCY_TEST = ROOT / "tests/test_forgejo_author_concurrency.py"
REQUEST_CONTENT = '''"""One complete request to the Forgejo author task service."""

from dataclasses import dataclass
from typing import MutableMapping

from a2a_server.forgejo_author_types import RoutingDecision, TaskData


@dataclass(frozen=True)
class AuthorTaskRequest:
    """Validated controller inputs needed to create an author task."""

    task_data: TaskData
    metadata: MutableMapping[str, object]
    routing: RoutingDecision
    workspace_id: str
    forgejo_token: str
'''
SERVICE_CONTENT = '''"""Serialized creation service for Forgejo author tasks."""

from a2a_server.forgejo_author_lock import serialized
from a2a_server.forgejo_author_request import AuthorTaskRequest
from a2a_server.forgejo_author_task import prepare
from a2a_server.forgejo_author_types import TaskBridge, WorkerValidator
from a2a_server.forgejo_author_verification import verify


async def create(
    bridge: TaskBridge,
    request: AuthorTaskRequest,
    validate_worker: WorkerValidator,
) -> object:
    """Verify, serialize, reuse, or durably create one author task."""
    metadata = request.metadata
    await verify(metadata, request.forgejo_token)
    async with serialized(metadata):
        task_id, existing = await prepare(metadata)
        if existing is not None:
            return existing
        await validate_worker(metadata, strict=True)
        task_data = request.task_data
        task_data.metadata = metadata
        task_data.routing = request.routing
        result = await bridge.create_task(
            codebase_id=request.workspace_id,
            title=task_data.title,
            prompt=task_data.prompt,
            agent_type=task_data.agent_type,
            priority=task_data.priority,
            metadata=metadata,
            task_id=task_id,
            require_persistence=True,
        )
        if result is None:
            raise RuntimeError('verified author task could not be durably persisted')
        return result
'''
FIXTURE_CONTENT = '''"""Compact request fixture for Forgejo author service tests."""

from types import SimpleNamespace

from a2a_server.forgejo_author_request import AuthorTaskRequest


def request(metadata=None, token='token') -> AuthorTaskRequest:
    """Build a minimal structurally typed author service request."""
    return AuthorTaskRequest(
        task_data=SimpleNamespace(
            title='review', prompt='data', agent_type='build', priority=1
        ),
        metadata=metadata or {},
        routing=SimpleNamespace(model_ref=None),
        workspace_id='global',
        forgejo_token=token,
    )
'''


def main() -> None:
    """Write request/service modules and update controller plus tests."""
    REQUEST.write_text(REQUEST_CONTENT)
    SERVICE.write_text(SERVICE_CONTENT)
    FIXTURE.write_text(FIXTURE_CONTENT)
    monitor = MONITOR.read_text()
    old_import = '        from .forgejo_author_service import create as create_author_task\n'
    new_import = '''        from a2a_server.forgejo_author_request import AuthorTaskRequest
        from a2a_server.forgejo_author_service import create as create_author_task
'''
    monitor = monitor.replace(old_import, new_import)
    old_call = '''            return await create_author_task(
                bridge,
                task_data,
                routed_metadata,
                routing_decision,
                effective_workspace_id,
                _validate_target_worker_is_available,
                forgejo_token,
            )
'''
    new_call = '''            request = AuthorTaskRequest(
                task_data=task_data,
                metadata=routed_metadata,
                routing=routing_decision,
                workspace_id=effective_workspace_id,
                forgejo_token=forgejo_token,
            )
            return await create_author_task(
                bridge, request, _validate_target_worker_is_available
            )
'''
    if new_call not in monitor:
        if monitor.count(old_call) != 1:
            raise RuntimeError('author service controller call anchor is missing')
        monitor = monitor.replace(old_call, new_call, 1)
    MONITOR.write_text(monitor)
    service_test = SERVICE_TEST.read_text()
    service_test = service_test.replace('from types import SimpleNamespace\n\n', '')
    service_test = service_test.replace(
        'import a2a_server.forgejo_author_service as service\n',
        'import a2a_server.forgejo_author_service as service\nfrom tests.forgejo_service_fixture import request\n',
    )
    start = service_test.find('    task = await service.create(\n')
    end = service_test.find('    assert task[', start)
    if start >= 0 and end >= 0:
        call = "    task = await service.create(Bridge(), request({'model': 'test'}, 'forgejo-token'), validate)\n"
        service_test = service_test[:start] + call + service_test[end:]
    SERVICE_TEST.write_text(service_test)
    concurrency = CONCURRENCY_TEST.read_text()
    concurrency = concurrency.replace('from types import SimpleNamespace\n\n', '')
    concurrency = concurrency.replace(
        'import a2a_server.forgejo_author_service as service\n',
        'import a2a_server.forgejo_author_service as service\nfrom tests.forgejo_service_fixture import request\n',
    )
    start = concurrency.find('    task_data = SimpleNamespace(\n')
    end = concurrency.find('    results = await asyncio.gather', start)
    if start >= 0 and end >= 0:
        calls = '''    calls = [service.create(Bridge(), request(), validate) for _ in range(2)]
'''
        concurrency = concurrency[:start] + calls + concurrency[end:]
    CONCURRENCY_TEST.write_text(concurrency)


if __name__ == '__main__':
    main()