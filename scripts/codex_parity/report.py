"""Benchmark report assembly and persistence."""

from datetime import datetime, timezone
from pathlib import Path
import json

from .case import BenchmarkCase
from .process import run
from .result import AgentResult
from .spec import AgentSpec
from .summary import compute


def write(
    root: Path,
    cases: list[BenchmarkCase],
    specs: list[AgentSpec],
    results: dict[str, list[AgentResult]],
) -> Path:
    """Write the complete comparison report without removing evidence."""
    document: dict[str, object] = {
        "run_date": datetime.now(timezone.utc).isoformat(),
        "agents": [_agent(spec) for spec in specs],
        "summary": compute(results),
        "cases": [
            {"id": case.identifier, "results": [item.document() for item in results[case.identifier]]}
            for case in cases
        ],
    }
    path = root / "report.json"
    path.write_text(json.dumps(document, indent=2) + "\n")
    return path


def _agent(spec: AgentSpec) -> dict[str, object]:
    version = run([spec.binary, "--version"], Path.cwd(), 10)
    return {"agent": spec.name, "model": spec.model, "version": version.stdout.strip()}
