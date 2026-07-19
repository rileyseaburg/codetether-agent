"""Extract shared test fixtures before they approach the line ceiling."""

from pathlib import Path

ROOT = Path("/home/riley/A2A-Server-MCP/tests")
METADATA = ROOT / "forgejo_metadata.py"
PROTOCOL = ROOT / "test_forgejo_author_protocol.py"
BRIDGE_FIXTURE = ROOT / "agent_bridge_fixtures.py"
BRIDGE_TEST = ROOT / "test_agent_bridge_required_persistence.py"
METADATA_CONTENT = '''"""Shared valid metadata for Forgejo author protocol tests."""

from a2a_server.forgejo_author_identity import canonical_identity
from a2a_server.forgejo_conversation_identity import conversation_id


def metadata() -> dict[str, object]:
    """Build one complete, internally consistent protocol envelope."""
    target = canonical_identity('forge.example', 'alice', 'default')
    context = conversation_id('owner/repo', 42, target)
    return {
        'protocol': 'codetether.forgejo-author.v1',
        'source': 'forgejo-pr-review',
        'workflow_stage': 'forgejo-author-review',
        'forgejo_host': 'forge.example',
        'forgejo_author_login': 'alice',
        'agent_slot': 'default',
        'target_agent_name': target,
        'repo': 'owner/repo',
        'pr_number': 42,
        'pr_head_sha': 'a' * 40,
        'resume_session_id': 'author-session',
        'author_provenance_id': 'ctprov_1234567890abcdef',
        'provenance_verified': True,
        'preserve_session_workspace': True,
        'author_agent_identity': target,
        'head_sha': 'a' * 40,
        'git_signer': 'forgejo:alice',
        'context_id': context,
        'conversation_id': context,
    }
'''
BRIDGE_CONTENT = '''"""Agent bridge fixtures with controlled persistence outcomes."""

from a2a_server.agent_bridge import AgentBridge


def bridge_with_save_result(saved: bool) -> AgentBridge:
    """Build a minimal bridge whose durable save has a fixed outcome."""
    bridge = AgentBridge.__new__(AgentBridge)
    bridge._tasks = {}
    bridge._codebase_tasks = {}

    async def save(_task: object) -> bool:
        return saved

    bridge._save_task = save
    return bridge
'''


def main() -> None:
    """Write fixtures and remove duplicated definitions from tests."""
    METADATA.write_text(METADATA_CONTENT)
    BRIDGE_FIXTURE.write_text(BRIDGE_CONTENT)
    protocol = PROTOCOL.read_text()
    start = protocol.find('def metadata():\n')
    end = protocol.find('\n\ndef test_identity_matches', start)
    if start >= 0 and end >= 0:
        protocol = protocol[:start] + protocol[end + 2:]
    protocol = protocol.replace(
        'from a2a_server.forgejo_conversation_identity import conversation_id\n', ''
    )
    helper_import = 'from tests.forgejo_metadata import metadata\n'
    if helper_import not in protocol:
        protocol = protocol.replace(
            'from a2a_server.forgejo_author_task import task_identity\n',
            'from a2a_server.forgejo_author_task import task_identity\n' + helper_import,
        )
    PROTOCOL.write_text(protocol)
    bridge = BRIDGE_TEST.read_text()
    start = bridge.find('def bridge_with_save_result(saved):\n')
    end = bridge.find('\n\n@pytest.mark.asyncio', start)
    if start >= 0 and end >= 0:
        bridge = bridge[:start] + bridge[end + 2:]
    bridge = bridge.replace(
        'from a2a_server.agent_bridge import AgentBridge\n',
        'from tests.agent_bridge_fixtures import bridge_with_save_result\n',
    )
    BRIDGE_TEST.write_text(bridge)
    for path in ROOT.glob('test_forgejo_*.py'):
        text = path.read_text().replace(
            'from tests.test_forgejo_author_protocol import metadata',
            'from tests.forgejo_metadata import metadata',
        )
        path.write_text(text)


if __name__ == "__main__":
    main()