//! Pending approval display snapshot.

use crate::approval::LiveApprovalRequest;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ApprovalSnapshot {
    pub(crate) id: String,
    pub(crate) tool: String,
    pub(crate) action: String,
    pub(crate) resource: String,
    pub(crate) reason: String,
}

impl From<LiveApprovalRequest> for ApprovalSnapshot {
    fn from(request: LiveApprovalRequest) -> Self {
        Self {
            id: request.approval_id,
            tool: request.tool,
            action: request.action,
            resource: request.resource,
            reason: request.reason,
        }
    }
}
