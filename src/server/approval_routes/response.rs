use crate::approval::{ApprovalDecision, ApprovalReceipt, ApprovalRequest, ApprovalStatus};
use serde_json::{Value, json};

pub(super) fn pending(request: &ApprovalRequest, tool_call_id: Option<String>) -> Value {
    json!({
        "approval_id": request.id,
        "status": ApprovalStatus::Pending,
        "request": request,
        "tool_call_id": tool_call_id
    })
}

pub(super) fn decided(
    approval_id: &str,
    status: ApprovalStatus,
    decision: Option<ApprovalDecision>,
    receipt: Option<ApprovalReceipt>,
    delivered: bool,
) -> Value {
    json!({
        "approval_id": approval_id,
        "status": status,
        "decision": decision,
        "receipt": receipt,
        "live_decision_delivered": delivered
    })
}
