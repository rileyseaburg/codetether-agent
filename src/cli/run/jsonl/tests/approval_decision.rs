use super::{RunEvent, event_value};
use crate::session::thread_store::ThreadEvent;

#[test]
fn approval_decided_has_decision_status() {
    let value = event_value(&RunEvent::ApprovalDecided {
        approval_id: "approval-1",
        decision_id: "decision-1",
        status: "approved",
        timestamp_ms: 7,
    });
    assert_eq!(value["type"], "run.approval_decided");
    assert_eq!(value["approval_id"], "approval-1");
    assert_eq!(value["status"], "approved");
}

#[test]
fn thread_approval_decided_writes_jsonl() {
    let event = ThreadEvent {
        event_id: "event-1".into(),
        thread_id: "thread-1".into(),
        session_id: "session-1".into(),
        turn_id: "turn-1".into(),
        kind: "approval.decided".into(),
        timestamp_ms: 7,
        payload: serde_json::json!({
            "approval_id": "approval-1",
            "decision_id": "decision-1",
            "status": "denied"
        }),
    };
    let mut out = Vec::new();
    assert!(super::super::thread::write_thread_event_to(&mut out, &event).unwrap());
    let value: serde_json::Value = serde_json::from_slice(&out).unwrap();
    assert_eq!(value["type"], "run.approval_decided");
    assert_eq!(value["status"], "denied");
}
