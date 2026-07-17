"""Execute and evaluate one agent in one retained workspace."""

from pathlib import Path

from . import commands
from .case import BenchmarkCase
from .checks import evaluate
from .process import run
from .result import AgentResult
from .spec import AgentSpec
from .workspace import capture, prepare


def execute(case: BenchmarkCase, spec: AgentSpec, root: Path) -> AgentResult:
    """Run an agent, retain raw evidence, and apply acceptance checks."""
    workspace = root / "workspaces" / case.identifier / spec.name
    evidence = root / "evidence" / case.identifier / spec.name
    evidence.mkdir(parents=True)
    prepare(case.fixture, workspace)
    outcome = run(
        commands.agent(spec, workspace, case.prompt), workspace, case.timeout_seconds
    )
    stdout_path = evidence / "stdout.jsonl"
    stderr_path = evidence / "stderr.log"
    stdout_path.write_text(outcome.stdout)
    stderr_path.write_text(outcome.stderr)
    check_results = evaluate(case, workspace)
    changed_files, patch = capture(workspace)
    patch_path = evidence / "changes.patch"
    patch_path.write_text(patch)
    passed = outcome.exit_code == 0 and all(check.passed for check in check_results)
    return AgentResult(
        spec.name, spec.model, passed, outcome.exit_code, outcome.timed_out,
        outcome.duration_seconds, changed_files, check_results,
        str(stdout_path), str(stderr_path), str(patch_path),
    )
