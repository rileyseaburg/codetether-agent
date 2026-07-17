"""Aggregate task-quality and timing metrics."""

from typing import TypedDict

from .result import AgentResult


class AgentSummary(TypedDict):
    """Aggregate metrics for one agent."""

    attempted: int
    passed: int
    pass_rate: float
    timeouts: int
    total_agent_seconds: float


def compute(results: dict[str, list[AgentResult]]) -> dict[str, object]:
    """Compute per-agent pass rates and the CodeTether parity gap."""
    names = ("codex", "codetether")
    agents = {name: _agent(name, results) for name in names}
    codex_rate = agents["codex"]["pass_rate"]
    codetether_rate = agents["codetether"]["pass_rate"]
    return {
        "agents": agents,
        "codetether_pass_rate_gap": codetether_rate - codex_rate,
    }


def _agent(name: str, results: dict[str, list[AgentResult]]) -> AgentSummary:
    selected = [
        result
        for case_results in results.values()
        for result in case_results
        if result.agent == name
    ]
    passed = sum(result.passed for result in selected)
    attempted = len(selected)
    return {
        "attempted": attempted,
        "passed": passed,
        "pass_rate": passed / attempted if attempted else 0.0,
        "timeouts": sum(result.timed_out for result in selected),
        "total_agent_seconds": sum(result.duration_seconds for result in selected),
    }
