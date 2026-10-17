//! Structured event regression case.
use super::*;

#[test]
fn secrets_inside_arrays_are_redacted() {
    let event = tool_call_event(
        "bash",
        &serde_json::json!({"items": [{"password": "hunter2"}]}),
    );

    assert_eq!(
        event["payload"]["arguments"]["items"][0]["password"],
        serde_json::json!("[redacted]")
    );
}
