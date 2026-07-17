"""Bounded child-process execution."""

from dataclasses import dataclass
from pathlib import Path
import subprocess
import time


@dataclass(frozen=True)
class ProcessResult:
    """Captured process outcome, including timeout state."""

    exit_code: int
    duration_seconds: float
    stdout: str
    stderr: str
    timed_out: bool


def run(argv: list[str], cwd: Path, timeout: int) -> ProcessResult:
    """Execute one command with captured output and a hard timeout."""
    started = time.monotonic()
    try:
        result = subprocess.run(
            argv, cwd=cwd, text=True, capture_output=True, timeout=timeout, check=False
        )
        return ProcessResult(
            result.returncode, time.monotonic() - started,
            result.stdout, result.stderr, False,
        )
    except subprocess.TimeoutExpired as error:
        return ProcessResult(
            124, time.monotonic() - started,
            _text(error.stdout), _text(error.stderr), True,
        )


def _text(value: str | bytes | None) -> str:
    if isinstance(value, bytes):
        return value.decode(errors="replace")
    return value or ""
