use crate::session::thread_store::ThreadEvent;

use super::super::thread::write_thread_event_to;

#[test]
fn mapped_tool_thread_event_serializes_rich_jsonl_tool() {
    let event = ThreadEvent {
        event_id: "event-1".into(),
        thread_id: "thread-1".into(),
        session_id: "session-1".into(),
        turn_id: "turn-1".into(),
        kind: "tool.completed".into(),
        timestamp_ms: 123,
        payload: serde_json::json!({
            "item_id": "item-1", "tool_call_id": "tool-1",
            "name": "bash", "success": true, "duration_ms": 3, "output": "ok"
        }),
    };
    let mut out = Vec::new();
    assert!(write_thread_event_to(&mut out, &event).unwrap());
    let value: serde_json::Value = serde_json::from_slice(&out).unwrap();
    assert_eq!(value["type"], "run.tool_call_completed");
    assert_eq!(value["tool_call_id"], "tool-1");
    assert_eq!(value["name"], "bash");
    assert_eq!(value["success"], true);
    assert_eq!(value["duration_ms"], 3);
    assert_eq!(value["output"], "ok");
}
