"""Require a durable write before replaying a cached deterministic task."""

from pathlib import Path

PATH = Path("/home/riley/A2A-Server-MCP/a2a_server/agent_bridge.py")
OLD = """        if task_id:
            cached = self._tasks.get(task_id)
            if cached:
                return cached
        else:
            task_id = str(uuid.uuid4())
"""
NEW = """        if task_id:
            cached = self._tasks.get(task_id)
            if cached:
                if require_persistence and not await self._save_task(cached):
                    return None
                return cached
        else:
            task_id = str(uuid.uuid4())
"""


def main() -> None:
    """Replace exactly one unsafe cached-task return."""
    text = PATH.read_text()
    if NEW in text:
        return
    if text.count(OLD) != 1:
        raise RuntimeError("cached persistence anchor missing or ambiguous")
    PATH.write_text(text.replace(OLD, NEW, 1))


if __name__ == "__main__":
    main()