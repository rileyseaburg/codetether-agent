"""Benchmark case definition."""

from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class BenchmarkCase:
    """One fixture, prompt, and deterministic acceptance contract."""

    identifier: str
    fixture: Path
    prompt: str
    checks: tuple[str, ...]
    protected: tuple[str, ...]
    timeout_seconds: int
