use crate::approval::ApprovalDecision;

use super::mapper;

#[test]
fn approval_decision_maps_to_thread_event() {
    let mut mapper = mapper();
    let decision = ApprovalDecision::approve("approval-1", "user", "ok");
    let event = mapper.approval_decided(&decision);
    assert_eq!(event.kind, "approval.decided");
    assert_eq!(event.payload["approval_id"], "approval-1");
    assert_eq!(event.payload["decision_id"], decision.id);
    assert_eq!(event.payload["status"], "approved");
}
