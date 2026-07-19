"""Extract durable task serialization from the oversized external bridge."""

from pathlib import Path

ROOT = Path("/home/riley/A2A-Server-MCP/a2a_server")
BRIDGE = ROOT / "agent_bridge.py"
RECORD = ROOT / "agent_task_record.py"
PERSIST = ROOT / "agent_task_persistence.py"
RECORD_CONTENT = '''"""PostgreSQL records for in-memory agent tasks."""

from __future__ import annotations

from datetime import datetime, timezone
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from .agent_bridge import AgentTask


def build(task: AgentTask) -> dict[str, object]:
    """Build the canonical durable record for an agent task."""
    metadata = dict(task.metadata or {})
    optional = (
        ('model', task.model),
        ('model_ref', task.model_ref),
        ('target_agent_name', task.target_agent_name),
        ('model_used', task.model_used),
    )
    for key, value in optional:
        if value and key not in metadata:
            metadata[key] = value
    return {
        'id': task.id,
        'workspace_id': task.codebase_id,
        'codebase_id': task.codebase_id,
        'title': task.title,
        'prompt': task.prompt,
        'agent_type': task.agent_type,
        'status': task.status.value,
        'priority': task.priority,
        'worker_id': None,
        'result': task.result,
        'error': task.error,
        'metadata': metadata,
        'tenant_id': metadata.get('tenant_id'),
        'created_at': task.created_at.isoformat(),
        'updated_at': datetime.now(timezone.utc).isoformat(),
        'started_at': task.started_at.isoformat() if task.started_at else None,
        'completed_at': task.completed_at.isoformat() if task.completed_at else None,
    }
'''
PERSIST_CONTENT = '''"""Fail-closed PostgreSQL persistence for agent tasks."""

from __future__ import annotations

import logging
from typing import TYPE_CHECKING

from . import database as db
from .agent_task_record import build

if TYPE_CHECKING:
    from .agent_bridge import AgentTask

logger = logging.getLogger(__name__)


async def save(task: AgentTask) -> bool:
    """Persist a task and propagate the database write result."""
    try:
        return await db.db_upsert_task(build(task))
    except Exception as error:
        logger.error('Failed to save task to PostgreSQL: %s', error)
        return False
'''


def main() -> None:
    """Write persistence modules and replace the legacy method body."""
    RECORD.write_text(RECORD_CONTENT)
    PERSIST.write_text(PERSIST_CONTENT)
    text = BRIDGE.read_text()
    import_line = 'from .agent_task_persistence import save as save_agent_task\n'
    if import_line not in text:
        anchor = 'from . import database as db\n'
        if text.count(anchor) != 1:
            raise RuntimeError('agent task persistence import anchor is missing')
        text = text.replace(anchor, anchor + import_line, 1)
    start_marker = '    async def _save_task(self, task: AgentTask) -> bool:\n'
    end_marker = '    def _task_from_db_row(self, row: Dict[str, Any]) -> AgentTask:\n'
    if start_marker in text:
        start = text.index(start_marker)
        end = text.index(end_marker, start)
        method = '''    async def _save_task(self, task: AgentTask) -> bool:
        """Save or update a task in PostgreSQL."""
        return await save_agent_task(task)

'''
        text = text[:start] + method + text[end:]
    BRIDGE.write_text(text)


if __name__ == "__main__":
    main()