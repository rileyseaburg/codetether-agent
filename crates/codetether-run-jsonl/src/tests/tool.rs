//! Tool-call lifecycle output.
use super::{RunEvent, event_value};

#[test]
fn tool_call_started_has_ids_and_timestamp() {
    let value = event_value(&RunEvent::ToolCallStarted {
        item_id: "item-1",
        tool_call_id: "call-1",
        timestamp_ms: 271,
        name: Some("bash"),
        arguments: Some(r#"{"command":"date"}"#),
    });
    assert_eq!(value["type"], "run.tool_call_started");
    assert_eq!(value["item_id"], "item-1");
    assert_eq!(value["tool_call_id"], "call-1");
    assert_eq!(value["timestamp_ms"], 271);
    assert_eq!(value["name"], "bash");
    assert_eq!(value["arguments"], r#"{"command":"date"}"#);
}

#[test]
fn tool_call_completed_has_ids_and_timestamp() {
    let value = event_value(&RunEvent::ToolCallCompleted {
        item_id: "item-2",
        tool_call_id: "call-2",
        timestamp_ms: 272,
        name: Some("bash"),
        success: Some(true),
        duration_ms: Some(9),
        output: Some("ok"),
    });
    assert_eq!(value["type"], "run.tool_call_completed");
    assert_eq!(value["item_id"], "item-2");
    assert_eq!(value["tool_call_id"], "call-2");
    assert_eq!(value["name"], "bash");
    assert_eq!(value["success"], true);
    assert_eq!(value["duration_ms"], 9);
    assert_eq!(value["output"], "ok");
}
