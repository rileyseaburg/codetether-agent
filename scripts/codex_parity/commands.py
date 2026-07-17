"""Build non-interactive commands for both agents."""

from pathlib import Path

from .spec import AgentSpec


def agent(spec: AgentSpec, workspace: Path, prompt: str) -> list[str]:
    """Return a full non-interactive agent invocation."""
    if spec.name == "codex":
        return [
            spec.binary, "exec", "--cd", str(workspace), "--sandbox",
            "workspace-write", "--skip-git-repo-check", "--ephemeral",
            "--json", "--model", spec.model, prompt,
        ]
    return [
        spec.binary, "run", "--format", "jsonl", "--access-mode", "approve",
        "--max-steps", str(spec.max_steps), "--model", spec.model, prompt,
        "--", str(workspace),
    ]
