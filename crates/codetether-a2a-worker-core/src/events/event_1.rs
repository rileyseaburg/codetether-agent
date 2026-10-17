//! Structured event regression case.
use super::*;

#[test]
fn tool_call_carries_name_and_arguments() {
    let event = tool_call_event("bash", &serde_json::json!({"command": "ls -la"}));

    assert_eq!(event["type"], serde_json::json!("tool.call"));
    assert_eq!(event["payload"]["name"], serde_json::json!("bash"));
    assert_eq!(
        event["payload"]["arguments"]["command"],
        serde_json::json!("ls -la")
    );
}
