use super::super::prompt::append_guardrails_for_cwd;
use std::path::Path;

fn prompt() -> String {
    append_guardrails_for_cwd("base".to_string(), Path::new("."))
}

#[test]
fn requires_exact_model_evidence_for_unavailability() {
    let prompt = prompt();
    assert!(prompt.contains("provider rejects the exact resolved model"));
    assert!(prompt.contains("both requested and resolved identifiers"));
}

#[test]
fn prevents_false_cross_runtime_corroboration() {
    let prompt = prompt();
    assert!(prompt.contains("do not claim all runtimes rejected the exact model"));
    assert!(prompt.contains("Do not treat direct-agent and swarm failures as corroboration"));
}

#[test]
fn classifies_zero_tool_calls_as_unknown_startup_failure() {
    let prompt = prompt();
    assert!(prompt.contains("zero tool calls is an unknown startup failure"));
    assert!(prompt.contains("next discriminating diagnostic"));
}

#[test]
fn foregrounds_observed_harness_defects() {
    let prompt = prompt();
    assert!(prompt.contains("Foreground observed harness routing"));
    assert!(prompt.contains("Rank conclusions by direct evidence"));
}
