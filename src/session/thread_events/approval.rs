//! Approval request thread-event mapping.

use serde_json::json;

use crate::approval::{ApprovalDecision, ApprovalStatus, LiveApprovalRequest};
use crate::session::thread_store::ThreadEvent;

use super::mapper::ThreadEventMapper;

impl ThreadEventMapper {
    pub(super) fn approval_requested(&mut self, request: &LiveApprovalRequest) -> ThreadEvent {
        self.event(
            "approval.requested",
            json!({
                "approval_id": request.approval_id,
                "tool_call_id": request.tool_call_id,
                "tool": request.tool,
                "action": request.action,
                "resource": request.resource,
                "reason": request.reason,
                "proposed_execpolicy_amendment": &request.proposed_execpolicy_amendment,
                "available_decisions": &request.available_decisions,
            }),
        )
    }

    /// Map a persisted approval decision.
    pub fn approval_decided(&mut self, decision: &ApprovalDecision) -> ThreadEvent {
        self.event(
            "approval.decided",
            json!({
                "approval_id": &decision.request_id,
                "decision_id": &decision.id,
                "status": status_name(decision.status),
                "decided_by": &decision.decided_by,
                "reason": &decision.reason,
                "decided_at": decision.decided_at.to_rfc3339(),
            }),
        )
    }
}

fn status_name(status: ApprovalStatus) -> &'static str {
    match status {
        ApprovalStatus::Pending => "pending",
        ApprovalStatus::Approved => "approved",
        ApprovalStatus::Denied => "denied",
    }
}
