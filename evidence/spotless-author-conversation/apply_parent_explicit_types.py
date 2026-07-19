"""Replace loose Any annotations in the external protocol with contracts."""

from pathlib import Path

ROOT = Path("/home/riley/A2A-Server-MCP/a2a_server")
TYPES = ROOT / "forgejo_author_types.py"
TYPES_CONTENT = '''"""Structural types used by the Forgejo author task service."""

from typing import MutableMapping, Optional, Protocol


class TaskData(Protocol):
    """HTTP-independent fields required to construct an author task."""

    title: str
    prompt: str
    agent_type: str
    priority: int


class RoutingDecision(Protocol):
    """Resolved model route required by task persistence."""

    model_ref: Optional[str]


class TaskBridge(Protocol):
    """Task persistence boundary used by the author service."""

    async def create_task(self, **kwargs: object) -> Optional[object]: ...


class WorkerValidator(Protocol):
    """Strict worker-availability validation boundary."""

    async def __call__(
        self, metadata: MutableMapping[str, object], *, strict: bool
    ) -> None: ...
'''


def edit(name: str, replacements: tuple[tuple[str, str], ...]) -> None:
    path = ROOT / name
    text = path.read_text()
    for old, new in replacements:
        if old in text:
            text = text.replace(old, new)
        elif new not in text:
            raise RuntimeError(f"explicit type anchor missing: {path}: {old}")
    path.write_text(text)


def main() -> None:
    """Install structural contracts and tighten every new module."""
    TYPES.write_text(TYPES_CONTENT)
    edit('forgejo_author_identity.py', (
        ('from typing import Any, Dict', 'from typing import Mapping'),
        ('metadata: Dict[str, Any]', 'metadata: Mapping[str, object]'),
    ))
    edit('forgejo_conversation_identity.py', (
        ('from typing import Any, Dict', 'from typing import Mapping'),
        ('pr_number: Any', 'pr_number: object'),
        ('metadata: Dict[str, Any]', 'metadata: Mapping[str, object]'),
    ))
    edit('forgejo_author_task.py', (
        ('from typing import Any, Dict, Optional, Tuple',
         'from typing import Dict, Mapping, MutableMapping, Optional, Tuple'),
        ('async def prepare(metadata: Dict[str, Any]) -> Tuple[str, Optional[Dict[str, Any]]]:',
         'async def prepare(\n    metadata: MutableMapping[str, object],\n) -> Tuple[str, Optional[Dict[str, object]]]:'),
        ('metadata: Dict[str, Any]', 'metadata: Mapping[str, object]'),
    ))
    edit('forgejo_author_lock.py', (
        ('from typing import Any, AsyncIterator, Dict',
         'from typing import AsyncIterator, Mapping'),
        ('metadata: Dict[str, Any]', 'metadata: Mapping[str, object]'),
    ))
    edit('forgejo_author_service.py', (
        ('from typing import Any, Awaitable, Callable, Dict\n\n', ''),
        ('from .forgejo_author_task import prepare\n\nWorkerValidator = Callable[..., Awaitable[None]]',
         'from .forgejo_author_task import prepare\nfrom .forgejo_author_types import (\n    RoutingDecision, TaskBridge, TaskData, WorkerValidator,\n)\nfrom typing import MutableMapping'),
        ('    bridge: Any,\n    task_data: Any,\n    metadata: Dict[str, Any],\n    routing_decision: Any,',
         '    bridge: TaskBridge,\n    task_data: TaskData,\n    metadata: MutableMapping[str, object],\n    routing_decision: RoutingDecision,'),
        ('):\n    """Create or replay exactly one durable task',
         ') -> object:\n    """Create or replay exactly one durable task'),
    ))


if __name__ == "__main__":
    main()