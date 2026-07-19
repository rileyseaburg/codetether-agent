"""Configure the database URL before importing the bridge test target."""

from pathlib import Path

PATH = Path("/home/riley/A2A-Server-MCP/tests/test_agent_bridge_required_persistence.py")
OLD = """import pytest

from a2a_server.agent_bridge import AgentBridge
"""
NEW = """import os

import pytest

os.environ.setdefault('DATABASE_URL', 'postgresql://test:test@localhost/test')

from a2a_server.agent_bridge import AgentBridge
"""


def main() -> None:
    """Insert the test-only configuration exactly once."""
    text = PATH.read_text()
    if NEW in text:
        return
    if text.count(OLD) != 1:
        raise RuntimeError("bridge test import anchor missing or ambiguous")
    PATH.write_text(text.replace(OLD, NEW, 1))


if __name__ == "__main__":
    main()