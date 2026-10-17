use super::{ApprovalDecision, ApprovalRequest};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Verified approval grant for a specific tool/action/resource tuple.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApprovalReceipt {
    /// Approval request id accepted as `approval_id`.
    pub approval_id: String,
    /// Decision event id that granted the approval.
    pub decision_id: String,
    /// Tool authorized by the request.
    pub tool: String,
    /// Action authorized by the request.
    pub action: String,
    /// Resource authorized by the request.
    pub resource: String,
    /// Actor that approved the request.
    pub decided_by: String,
    /// UTC timestamp when the request was approved.
    pub decided_at: DateTime<Utc>,
}

impl ApprovalReceipt {
    pub(crate) fn from_parts(request: &ApprovalRequest, decision: &ApprovalDecision) -> Self {
        Self {
            approval_id: request.id.clone(),
            decision_id: decision.id.clone(),
            tool: request.tool.clone(),
            action: request.action.clone(),
            resource: request.resource.clone(),
            decided_by: decision.decided_by.clone(),
            decided_at: decision.decided_at,
        }
    }
}
