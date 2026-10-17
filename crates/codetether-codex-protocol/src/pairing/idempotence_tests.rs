//! Reapplying request repair must not invent duplicate tool outcomes.

#[test]
fn complete_groups_are_unchanged_and_repair_is_idempotent() {
    let input = vec![
        serde_json::json!({"type":"function_call", "call_id":"done"}),
        serde_json::json!({"type":"function_call_output", "call_id":"done", "output":"real"}),
    ];
    assert_eq!(super::repair(input.clone()), input);
    let partial = vec![serde_json::json!({"type":"function_call", "call_id":"missing"})];
    let repaired = super::repair(partial);
    assert_eq!(super::repair(repaired.clone()), repaired);
}

#[test]
fn empty_input_stays_empty() {
    assert!(super::repair(Vec::new()).is_empty());
}
