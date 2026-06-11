use crate::approval::{ApprovalDecision, ApprovalRequest, ApprovalStatus};

pub(crate) fn print_request(request: &ApprovalRequest, status: ApprovalStatus) {
    println!("id: {}", request.id);
    println!("status: {}", status_label(status));
    println!("tool: {}", request.tool);
    println!("action: {}", request.action);
    println!("resource: {}", request.resource);
    println!("reason: {}", request.reason);
    println!("requested_at: {}", request.requested_at.to_rfc3339());
}

pub(crate) fn print_decision(decision: &ApprovalDecision) {
    println!("decision_id: {}", decision.id);
    println!("decided_by: {}", decision.decided_by);
    println!("decided_at: {}", decision.decided_at.to_rfc3339());
    println!("decision_reason: {}", decision.reason);
}

pub(crate) fn status_label(status: ApprovalStatus) -> &'static str {
    match status {
        ApprovalStatus::Pending => "pending",
        ApprovalStatus::Approved => "approved",
        ApprovalStatus::Denied => "denied",
    }
}
