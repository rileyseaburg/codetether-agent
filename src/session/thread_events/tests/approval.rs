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
    );
    let events = mapper.map_session_event(&SessionEvent::ApprovalRequest(request));
    assert_eq!(events[0].kind, "approval.requested");
    assert_eq!(events[0].payload["approval_id"], "approval-1");
    assert_eq!(events[0].payload["tool_call_id"], "call-1");
}
