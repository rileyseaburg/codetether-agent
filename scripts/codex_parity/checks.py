"""Deterministic acceptance and protected-file checks."""

from dataclasses import dataclass
from pathlib import Path

from .case import BenchmarkCase
from .process import run


@dataclass(frozen=True)
class CheckResult:
    """One deterministic acceptance result."""

    command: str
    passed: bool
    exit_code: int
    duration_seconds: float
    output: str


def evaluate(case: BenchmarkCase, workspace: Path) -> list[CheckResult]:
    """Run protected-file comparisons followed by manifest checks."""
    results = [_protected(case, workspace)]
    for command in case.checks:
        outcome = run(["bash", "-lc", command], workspace, case.timeout_seconds)
        output = outcome.stdout + outcome.stderr
        results.append(CheckResult(
            command, outcome.exit_code == 0, outcome.exit_code,
            outcome.duration_seconds, output,
        ))
    return results


def _protected(case: BenchmarkCase, workspace: Path) -> CheckResult:
    changed = [path for path in case.protected if not _same(case.fixture, workspace, path)]
    output = "" if not changed else "modified protected files: " + ", ".join(changed)
    return CheckResult("protected files unchanged", not changed, 0 if not changed else 1, 0.0, output)


def _same(fixture: Path, workspace: Path, relative: str) -> bool:
    original = fixture / relative
    candidate = workspace / relative
    return candidate.is_file() and candidate.read_bytes() == original.read_bytes()
