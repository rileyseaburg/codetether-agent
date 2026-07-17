"""Agent command specification."""

from dataclasses import dataclass
from typing import Literal


AgentName = Literal["codex", "codetether"]


@dataclass(frozen=True)
class AgentSpec:
    """Executable and model used for one side of the comparison."""

    name: AgentName
    binary: str
    model: str
    max_steps: int
