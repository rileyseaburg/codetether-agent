use crate::approval::LiveApprovalRequest;
use crate::session::SessionEvent;

use super::mapper;

#[test]
fn approval_request_maps_to_thread_event() {
    let mut mapper = mapper();
    let request = LiveApprovalRequest::new(
        "approval-1".into(),
        "call-1".into(),
        "bash".into(),
        "execute".into(),
        "bash:abc".into(),
        "runtime policy".into(),
    )
    .with_preview("cargo test --lib approval".into());
    let events = mapper.map_session_event(&SessionEvent::ApprovalRequest(request));
    assert_eq!(events[0].kind, "approval.requested");
    assert_eq!(events[0].payload["approval_id"], "approval-1");
    assert_eq!(events[0].payload["tool_call_id"], "call-1");
    assert_eq!(events[0].payload["preview"], "cargo test --lib approval");
}

#[test]
fn decision_metadata_maps_to_approval_decided() {
    let mut mapper = mapper();
    let decision = crate::approval::ApprovalDecision::approve("approval-1", "tui", "ok");
    let event = SessionEvent::ToolCallMetadata {
        tool_call_id: "call-1".into(),
        name: "bash".into(),
        metadata: serde_json::json!({"approval_decision": decision}),
    };
    let events = mapper.map_session_event(&event);
    assert_eq!(events.len(), 2);
    assert_eq!(events[1].kind, "approval.decided");
    assert_eq!(events[1].payload["approval_id"], "approval-1");
    assert_eq!(events[1].payload["status"], "approved");
}
