use crate::session::thread_store::ThreadEvent;

use super::super::thread::write_thread_event_to;

fn event(kind: &str, text: &str) -> ThreadEvent {
    ThreadEvent {
        event_id: "event-1".into(),
        thread_id: "thread-1".into(),
        session_id: "session-1".into(),
        turn_id: "turn-1".into(),
        kind: kind.into(),
        timestamp_ms: 123,
        payload: serde_json::json!({"item_id": "item-1", "text": text}),
    }
}

#[test]
fn text_delta_is_written_as_a_live_jsonl_chunk() {
    let mut out = Vec::new();
    assert!(write_thread_event_to(&mut out, &event("item.delta", "Hello")).unwrap());
    let value: serde_json::Value = serde_json::from_slice(&out).unwrap();
    assert_eq!(value["type"], "run.text_chunk");
    assert_eq!(value["text"], "Hello");
    assert_eq!(value["item_id"], "item-1");
}

#[test]
fn completed_text_is_distinct_from_a_generic_item_completion() {
    let mut out = Vec::new();
    assert!(write_thread_event_to(&mut out, &event("item.completed", "Hello")).unwrap());
    let value: serde_json::Value = serde_json::from_slice(&out).unwrap();
    assert_eq!(value["type"], "run.text_complete");
    assert_eq!(value["text"], "Hello");
}
