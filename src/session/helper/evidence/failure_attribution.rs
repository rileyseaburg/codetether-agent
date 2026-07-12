//! Evidence rules for attributing agent and provider failures.

const RULES: &str = "\
Failure attribution rules:
- Separate observed facts, unresolved hypotheses, and confirmed root causes.
- Claim model unavailability only when the provider rejects the exact resolved model; report both requested and resolved identifiers.
- Attribute every failure to its runtime and concrete diagnostic; do not claim all runtimes rejected the exact model without matching evidence.
- Do not treat direct-agent and swarm failures as corroboration unless their routing inputs, resolved identifiers, and failure categories match.
- A failed child with zero tool calls is an unknown startup failure until lifecycle evidence identifies the failing stage.
- Name a typed confirmed blocker or list unresolved hypotheses plus the next discriminating diagnostic.
- Foreground observed harness routing, initialization, or observability defects instead of shifting blame to model availability.
- Rank conclusions by direct evidence and use calibrated language when telemetry is incomplete.";

pub(super) fn render() -> &'static str {
    RULES
}

#[cfg(test)]
#[path = "failure_attribution_tests.rs"]
mod tests;
