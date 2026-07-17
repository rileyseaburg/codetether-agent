"""Serializable result records."""

from dataclasses import asdict, dataclass

from .checks import CheckResult


@dataclass(frozen=True)
class AgentResult:
    """Agent execution, checks, changes, and retained artifact paths."""

    agent: str
    model: str
    passed: bool
    exit_code: int
    timed_out: bool
    duration_seconds: float
    changed_files: list[str]
    checks: list[CheckResult]
    stdout_path: str
    stderr_path: str
    patch_path: str

    def document(self) -> dict[str, object]:
        """Convert the result to a JSON-compatible mapping."""
        return asdict(self)
