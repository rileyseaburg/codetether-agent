"""Apply the required-persistence guard to the external A2A worktree."""

from pathlib import Path

PATH = Path("/home/riley/A2A-Server-MCP/a2a_server/agent_bridge.py")
OLD = """        self._codebase_tasks[codebase_id].append(task_id)

        # Persist to database
        await self._save_task(task)
"""
NEW = """        self._codebase_tasks[codebase_id].append(task_id)

        # Persist to database
        saved = await self._save_task(task)
        if require_persistence and not saved:
            self._tasks.pop(task_id, None)
            self._codebase_tasks[codebase_id] = [
                value
                for value in self._codebase_tasks[codebase_id]
                if value != task_id
            ]
            return None
"""


def main() -> None:
    """Replace exactly one unguarded task-persistence call."""
    text = PATH.read_text()
    if NEW in text:
        return
    if text.count(OLD) != 1:
        raise RuntimeError("required-persistence anchor missing or ambiguous")
    PATH.write_text(text.replace(OLD, NEW, 1))


if __name__ == "__main__":
    main()