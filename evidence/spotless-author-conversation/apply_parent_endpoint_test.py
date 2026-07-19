"""Add HTTP-boundary tests for protocol-only credential extraction."""

from pathlib import Path

PATH = Path("/home/riley/A2A-Server-MCP/tests/test_forgejo_task_endpoint.py")
CONTENT = '''from types import SimpleNamespace

import pytest

import a2a_server.monitor_api as monitor


@pytest.mark.asyncio
async def test_protocol_endpoint_separates_bearer_and_forgejo_tokens(monkeypatch):
    task = monitor.AgentTaskCreate(
        title='review',
        prompt='data',
        metadata={'protocol': 'codetether.forgejo-author.v1'},
    )
    request = SimpleNamespace(
        headers={
            'authorization': 'Bearer codetether-secret',
            'x-forgejo-token': 'forgejo-secret',
        }
    )
    captured = []

    def scope(_request):
        return 'token:verified:fingerprint', 'tenant-a'

    async def create(*arguments):
        captured.append(arguments)
        return {'id': 'task'}

    monkeypatch.setattr(monitor, 'forgejo_request_scope', scope)
    monkeypatch.setattr(monitor, 'create_global_task', create)
    await monitor.create_global_task_endpoint(task, request)
    assert captured == [
        (task, 'forgejo-secret', 'token:verified:fingerprint', 'tenant-a')
    ]
    assert 'codetether-secret' not in str(captured)


@pytest.mark.asyncio
async def test_non_protocol_endpoint_keeps_public_compatibility(monkeypatch):
    task = monitor.AgentTaskCreate(title='legacy', prompt='data')
    request = SimpleNamespace(headers={})
    captured = []

    async def create(*arguments):
        captured.append(arguments)
        return {'id': 'task'}

    def unexpected_scope(_request):
        raise AssertionError('legacy task must not enter protocol authentication')

    monkeypatch.setattr(monitor, 'forgejo_request_scope', unexpected_scope)
    monkeypatch.setattr(monitor, 'create_global_task', create)
    await monitor.create_global_task_endpoint(task, request)
    assert captured == [(task, '', 'internal:global', None)]
'''


def main() -> None:
    """Write the focused controller contract tests."""
    PATH.write_text(CONTENT)


if __name__ == '__main__':
    main()