"""Top-level differential benchmark orchestration."""

from pathlib import Path
import json
import shlex

from . import commands
from .case import BenchmarkCase
from .report import write
from .result import AgentResult
from .run_agent import execute
from .spec import AgentSpec


def run(cases: list[BenchmarkCase], specs: list[AgentSpec], root: Path) -> Path:
    """Run every case for both agents and write a retained report."""
    if root.exists():
        raise FileExistsError(f"refusing to overwrite benchmark artifacts: {root}")
    root.mkdir(parents=True)
    results: dict[str, list[AgentResult]] = {}
    for case in cases:
        results[case.identifier] = [execute(case, spec, root) for spec in specs]
    return write(root, cases, specs, results)


def dry_run(cases: list[BenchmarkCase], specs: list[AgentSpec]) -> str:
    """Render commands and checks without invoking either model."""
    preview: list[dict[str, object]] = []
    for case in cases:
        workspace = Path("<artifacts>") / "workspaces" / case.identifier
        preview.append({
            "case": case.identifier,
            "commands": [
                shlex.join(commands.agent(spec, workspace / spec.name, case.prompt))
                for spec in specs
            ],
            "checks": list(case.checks),
            "protected": list(case.protected),
        })
    return json.dumps(preview, indent=2)
