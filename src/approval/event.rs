use super::{ApprovalDecision, ApprovalRequest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub(crate) enum ApprovalEvent {
    Request { request: ApprovalRequest },
    Decision { decision: ApprovalDecision },
}

impl From<ApprovalRequest> for ApprovalEvent {
    fn from(request: ApprovalRequest) -> Self {
        Self::Request { request }
    }
}

impl From<ApprovalDecision> for ApprovalEvent {
    fn from(decision: ApprovalDecision) -> Self {
        Self::Decision { decision }
    }
}
