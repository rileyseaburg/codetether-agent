"""Persistent isolated-workspace preparation and diff capture."""

from pathlib import Path
import shutil

from .process import run


def prepare(fixture: Path, target: Path) -> None:
    """Copy a fixture and establish its immutable git baseline."""
    shutil.copytree(fixture, target)
    for argv in (
        ["git", "init", "--quiet"],
        ["git", "config", "user.email", "parity@codetether.run"],
        ["git", "config", "user.name", "Parity Benchmark"],
        ["git", "add", "-A"],
        ["git", "commit", "--quiet", "-m", "benchmark baseline"],
    ):
        result = run(argv, target, 30)
        if result.exit_code != 0:
            raise RuntimeError(f"workspace setup failed: {result.stderr}")


def capture(target: Path) -> tuple[list[str], str]:
    """Return status lines and a patch that includes untracked files."""
    run(["git", "add", "-N", "."], target, 30)
    status = run(["git", "status", "--short"], target, 30)
    patch = run(["git", "diff", "--binary"], target, 30)
    return status.stdout.splitlines(), patch.stdout
