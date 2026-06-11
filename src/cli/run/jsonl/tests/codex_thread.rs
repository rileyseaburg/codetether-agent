use crate::session::thread_store::ThreadEvent;

use super::super::thread::codex_thread::write_codex_thread_event_to;

#[test]
fn codex_thread_adapter_preserves_thread_turn_and_item_ids() {
    let event = ThreadEvent {
        event_id: "event-1".into(),
        thread_id: "thread-1".into(),
        session_id: "session-1".into(),
        turn_id: "turn-1".into(),
        kind: "tool.started".into(),
        timestamp_ms: 123,
        payload: serde_json::json!({
            "item_id": "item-1",
            "tool_call_id": "call-1"
        }),
    };
    let mut out = Vec::new();
    assert!(write_codex_thread_event_to(&mut out, &event).unwrap());
    let value: serde_json::Value = serde_json::from_slice(&out).unwrap();
    assert_eq!(value["timestamp"], "1970-01-01T00:00:00.123Z");
    assert_eq!(value["type"], "tool.started");
    assert_eq!(value["payload"]["event_id"], "event-1");
    assert_eq!(value["payload"]["thread_id"], "thread-1");
    assert_eq!(value["payload"]["turn_id"], "turn-1");
    assert_eq!(value["payload"]["item_id"], "item-1");
}
