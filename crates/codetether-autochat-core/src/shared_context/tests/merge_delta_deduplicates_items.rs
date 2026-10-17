//! Existing shared-context regression.
use super::*;

#[test]
fn merge_delta_deduplicates_items() {
    let mut shared = SharedRelayContext::default();
    let delta = ContextDelta {
        facts: vec!["same".to_string(), "same".to_string()],
        decisions: Vec::new(),
        risks: Vec::new(),
        next_actions: Vec::new(),
        evidence: Vec::new(),
    };
    shared.merge_delta(&delta);
    assert_eq!(shared.facts.len(), 1);
}
