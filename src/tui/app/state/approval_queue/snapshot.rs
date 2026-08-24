//! Pending approval display snapshot.

use super::ApprovalReport;
use crate::approval::ExecPolicyAmendment;
use crate::approval::LiveApprovalRequest;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ApprovalSnapshot {
    pub(crate) id: String,
    pub(crate) tool: String,
    pub(crate) action: String,
    pub(crate) resource: String,
    pub(crate) reason: String,
    pub(crate) preview: Option<String>,
    pub(crate) amendment: Option<ExecPolicyAmendment>,
    pub(crate) report: ApprovalReport,
}

impl From<LiveApprovalRequest> for ApprovalSnapshot {
    fn from(request: LiveApprovalRequest) -> Self {
        Self {
            id: request.approval_id,
            tool: request.tool,
            action: request.action,
            resource: request.resource,
            reason: request.reason,
            preview: request.preview,
            amendment: request.proposed_execpolicy_amendment,
            report: ApprovalReport::default(),
        }
    }
}
