//! Structured event regression case.
use super::*;

#[test]
fn reasoning_is_its_own_event_type() {
    let event = reasoning_event("Checking the failing test first.");

    assert_eq!(event["type"], serde_json::json!("agent.reasoning"));
    assert_eq!(
        event["payload"]["content"],
        serde_json::json!("Checking the failing test first.")
    );
    assert_eq!(event["payload"]["role"], serde_json::json!("assistant"));
}
