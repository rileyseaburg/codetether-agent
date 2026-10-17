//! Structured event regression case.
use super::*;

#[test]
fn oversized_arguments_are_bounded() {
    let big = "y".repeat(MAX_ARG_BYTES * 2);
    let event = tool_call_event("write", &serde_json::json!({"content": big}));
    let carried = event["payload"]["arguments"]["content"].as_str().unwrap();

    assert!(carried.contains("[truncated]"));
    assert!(carried.len() <= MAX_ARG_BYTES + 32, "{}", carried.len());
}
