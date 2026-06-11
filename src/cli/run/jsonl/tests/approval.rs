use super::{RunEvent, event_value};
use crate::session::thread_store::ThreadEvent;

#[test]
fn approval_requested_has_ids_and_tool() {
    let value = event_value(&RunEvent::ApprovalRequested {
        approval_id: "approval-1",
        tool_call_id: "call-1",
        tool: "bash",
        action: "execute",
        timestamp_ms: 7,
        proposed_execpolicy_amendment: None,
        available_decisions: None,
    });
    assert_eq!(value["type"], "run.approval_requested");
    assert_eq!(value["approval_id"], "approval-1");
    assert_eq!(value["tool_call_id"], "call-1");
    assert_eq!(value["tool"], "bash");
}

#[test]
fn thread_approval_requested_writes_jsonl() {
    let event = ThreadEvent {
        event_id: "event-1".into(),
        thread_id: "thread-1".into(),
        session_id: "session-1".into(),
        turn_id: "turn-1".into(),
        kind: "approval.requested".into(),
        timestamp_ms: 7,
        payload: serde_json::json!({
            "approval_id": "approval-1",
            "tool_call_id": "call-1",
            "tool": "bash",
            "action": "execute",
            "proposed_execpolicy_amendment": ["cargo", "test"],
            "available_decisions": ["approved", "approved_for_session"]
        }),
    };
    let mut out = Vec::new();
    assert!(super::super::thread::write_thread_event_to(&mut out, &event).unwrap());
    let value: serde_json::Value = serde_json::from_slice(&out).unwrap();
    assert_eq!(value["type"], "run.approval_requested");
    assert_eq!(value["proposed_execpolicy_amendment"][0], "cargo");
    assert_eq!(value["available_decisions"][1], "approved_for_session");
}
