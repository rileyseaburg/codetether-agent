"""Normalize import grouping in the external author task service."""

from pathlib import Path

PATH = Path("/home/riley/A2A-Server-MCP/a2a_server/forgejo_author_service.py")
OLD = '''from .forgejo_author_lock import serialized
from .forgejo_author_task import prepare
from .forgejo_author_verification import verify
from .forgejo_author_types import (
    RoutingDecision, TaskBridge, TaskData, WorkerValidator,
)
from typing import MutableMapping
'''
NEW = '''from typing import MutableMapping

from .forgejo_author_lock import serialized
from .forgejo_author_task import prepare
from .forgejo_author_types import (
    RoutingDecision,
    TaskBridge,
    TaskData,
    WorkerValidator,
)
from .forgejo_author_verification import verify
'''


def main() -> None:
    """Replace the generated import block exactly once."""
    text = PATH.read_text()
    if NEW not in text:
        if text.count(OLD) != 1:
            raise RuntimeError("service import block is missing")
        PATH.write_text(text.replace(OLD, NEW, 1))


if __name__ == "__main__":
    main()