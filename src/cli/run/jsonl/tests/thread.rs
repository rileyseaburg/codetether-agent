use crate::session::thread_store::ThreadEvent;

use super::super::thread::write_thread_event_to;

fn event(kind: &str, payload: serde_json::Value) -> ThreadEvent {
    ThreadEvent {
        event_id: "event-1".into(),
        thread_id: "thread-1".into(),
        session_id: "session-1".into(),
        turn_id: "turn-1".into(),
        kind: kind.into(),
        timestamp_ms: 123,
        payload,
    }
}

#[test]
fn mapped_item_thread_event_serializes_as_jsonl_item() {
    let event = event("item.started", serde_json::json!({ "item_id": "item-1" }));
    let mut out = Vec::new();
    assert!(write_thread_event_to(&mut out, &event).unwrap());
    let value: serde_json::Value = serde_json::from_slice(&out).unwrap();
    assert_eq!(value["type"], "run.item_started");
    assert_eq!(value["item_id"], "item-1");
    assert_eq!(value["timestamp_ms"], 123);
}

#[test]
fn mapped_patch_thread_event_serializes_as_jsonl_patch() {
    let event = event(
        "patch.approval_required",
        serde_json::json!({ "approval_id": "approval-1", "patch_id": "patch-1" }),
    );
    let mut out = Vec::new();
    assert!(write_thread_event_to(&mut out, &event).unwrap());
    let value: serde_json::Value = serde_json::from_slice(&out).unwrap();
    assert_eq!(value["type"], "run.patch_approval_required");
    assert_eq!(value["approval_id"], "approval-1");
    assert_eq!(value["patch_id"], "patch-1");
}
