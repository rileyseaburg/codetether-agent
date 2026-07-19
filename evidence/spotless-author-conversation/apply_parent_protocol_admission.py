"""Reserve Forgejo author metadata for the verified service boundary."""

from pathlib import Path

ROOT = Path('/home/riley/A2A-Server-MCP')
ADMISSION = ROOT / 'a2a_server/forgejo_protocol_admission.py'
BRIDGE = ROOT / 'a2a_server/agent_bridge.py'
SERVICE = ROOT / 'a2a_server/forgejo_author_service.py'
TEST = ROOT / 'tests/test_forgejo_protocol_admission.py'
ADMISSION_CONTENT = '''"""Unforgeable in-process admission for verified Forgejo author tasks."""

from collections.abc import Mapping

PROTOCOL = 'codetether.forgejo-author.v1'
_ADMISSION = object()


def token() -> object:
    """Return the verified author service's private capability."""
    return _ADMISSION


def require(metadata: Mapping[str, object], admission: object | None) -> None:
    """Reject reserved protocol metadata without the private capability."""
    if metadata.get('protocol') == PROTOCOL and admission is not _ADMISSION:
        raise ValueError('Forgejo author tasks require verified protocol admission')
'''
TEST_CONTENT = '''import pytest

from a2a_server.agent_bridge import AgentBridge
from a2a_server.forgejo_protocol_admission import require, token


def test_reserved_protocol_requires_the_private_service_capability():
    metadata = {'protocol': 'codetether.forgejo-author.v1'}
    with pytest.raises(ValueError, match='verified protocol admission'):
        require(metadata, None)
    require(metadata, token())


@pytest.mark.asyncio
async def test_bridge_rejects_direct_reserved_protocol_creation():
    bridge = AgentBridge(agent_bin='codetether', auto_start=False)
    with pytest.raises(ValueError, match='verified protocol admission'):
        await bridge.create_task(
            codebase_id=None,
            title='forged',
            prompt='forged',
            agent_type='build',
            metadata={'protocol': 'codetether.forgejo-author.v1'},
        )
'''


def main() -> None:
    """Install central admission enforcement and its focused tests."""
    ADMISSION.write_text(ADMISSION_CONTENT)
    bridge = BRIDGE.read_text()
    import_line = '''from .forgejo_protocol_admission import require as require_protocol_admission
'''
    if import_line not in bridge:
        anchor = 'from .agent_task_persistence import save as save_agent_task\n'
        bridge = bridge.replace(anchor, anchor + import_line, 1)
    signature = '''        task_id: Optional[str] = None,
        require_persistence: bool = False,
'''
    admitted_signature = signature + '        protocol_admission: object | None = None,\n'
    if admitted_signature not in bridge:
        bridge = bridge.replace(signature, admitted_signature, 1)
    metadata_anchor = '        task_metadata = dict(metadata or {})\n'
    admission_call = metadata_anchor + '''        require_protocol_admission(task_metadata, protocol_admission)
'''
    if admission_call not in bridge:
        bridge = bridge.replace(metadata_anchor, admission_call, 1)
    BRIDGE.write_text(bridge)
    service = SERVICE.read_text()
    token_import = 'from a2a_server.forgejo_protocol_admission import token as admission_token\n'
    if token_import not in service:
        anchor = 'from a2a_server.forgejo_author_types import TaskBridge, WorkerValidator\n'
        service = service.replace(anchor, anchor + token_import, 1)
    persist_anchor = '            require_persistence=True,\n'
    admitted_call = persist_anchor + '            protocol_admission=admission_token(),\n'
    if admitted_call not in service:
        service = service.replace(persist_anchor, admitted_call, 1)
    SERVICE.write_text(service)
    TEST.write_text(TEST_CONTENT)


if __name__ == '__main__':
    main()