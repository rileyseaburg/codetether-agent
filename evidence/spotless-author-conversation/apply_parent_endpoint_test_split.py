"""Separate authenticated protocol and legacy endpoint compatibility tests."""

from pathlib import Path

ROOT = Path("/home/riley/A2A-Server-MCP/tests")
PROTOCOL = ROOT / "test_forgejo_task_endpoint.py"
LEGACY = ROOT / "test_legacy_task_endpoint.py"
LEGACY_CONTENT = '''from types import SimpleNamespace

import pytest

import a2a_server.monitor_api as monitor


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
    """Move the independent compatibility behavior to its own test file."""
    text = PROTOCOL.read_text()
    marker = '\n\n@pytest.mark.asyncio\nasync def test_non_protocol_endpoint_keeps_public_compatibility'
    index = text.find(marker)
    if index >= 0:
        PROTOCOL.write_text(text[:index] + '\n')
    LEGACY.write_text(LEGACY_CONTENT)


if __name__ == '__main__':
    main()