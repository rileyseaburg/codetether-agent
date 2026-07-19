"""Make the external bridge propagate durable-write failure faithfully."""

from pathlib import Path

BRIDGE = Path("/home/riley/A2A-Server-MCP/a2a_server/agent_bridge.py")
TEST = Path("/home/riley/A2A-Server-MCP/tests/test_agent_bridge_save_result.py")
OLD = '''            await db.db_upsert_task(
'''
NEW = '''            return await db.db_upsert_task(
'''
TAIL = '''            )
            return True
        except Exception as e:
'''
FIXED_TAIL = '''            )
        except Exception as e:
'''
TEST_CONTENT = '''import os
from types import SimpleNamespace

import pytest

os.environ.setdefault('DATABASE_URL', 'postgresql://test:test@localhost/test')

from a2a_server import agent_bridge
from a2a_server.agent_bridge import AgentBridge


@pytest.mark.asyncio
async def test_save_task_propagates_database_false(monkeypatch):
    async def failed_upsert(_task):
        return False

    monkeypatch.setattr(agent_bridge.db, 'db_upsert_task', failed_upsert)
    task = SimpleNamespace(
        metadata={}, model=None, model_ref=None, target_agent_name=None,
        model_used=None, id='task', codebase_id='global', title='title',
        prompt='prompt', agent_type='build', status=SimpleNamespace(value='pending'),
        priority=0, result=None, error=None,
        created_at=SimpleNamespace(isoformat=lambda: 'created'),
        started_at=None, completed_at=None,
    )
    assert await AgentBridge.__new__(AgentBridge)._save_task(task) is False
'''


def main() -> None:
    """Patch the bridge and write its direct regression test."""
    text = BRIDGE.read_text()
    if NEW not in text:
        if text.count(OLD) != 1:
            raise RuntimeError("bridge upsert anchor is missing")
        text = text.replace(OLD, NEW, 1)
    if TAIL in text:
        text = text.replace(TAIL, FIXED_TAIL, 1)
    BRIDGE.write_text(text)
    TEST.write_text(TEST_CONTENT)


if __name__ == "__main__":
    main()