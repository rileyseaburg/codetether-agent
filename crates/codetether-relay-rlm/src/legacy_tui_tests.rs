//! Existing TUI regression cases moved beside their implementation.
use crate::semantic::extract_semantic_handoff_from_rlm;

#[test]
fn extract_semantic_handoff_prefers_semantic_payload_answer() {
    let payload = r#"{"kind":"semantic","file":"relay_handoff","answer":"Ship phase 1, test API edge cases, then hand off to infra."}"#;
    assert_eq!(
        extract_semantic_handoff_from_rlm(payload),
        "Ship phase 1, test API edge cases, then hand off to infra."
    );
}

#[test]
fn extract_semantic_handoff_falls_back_to_trimmed_raw_answer() {
    assert_eq!(
        extract_semantic_handoff_from_rlm("  plain relay output  "),
        "plain relay output"
    );
}
