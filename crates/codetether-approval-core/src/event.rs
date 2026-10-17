use super::{ApprovalDecision, ApprovalRequest};
use serde::{Deserialize, Serialize};

/// One immutable record in the approval store's JSON Lines journal.
///
/// Consumers can replay requests and decisions without participating in live
/// session delivery. Serialization retains the `event` discriminator.
///
/// # Examples
/// ```
/// use codetether_approval_core::{ApprovalEvent, ApprovalRequest};
/// let event = ApprovalEvent::from(ApprovalRequest::new("write", "update", "a.txt", "save"));
/// match event {
///     ApprovalEvent::Request { request } => assert_eq!(request.resource, "a.txt"),
///     ApprovalEvent::Decision { .. } => panic!("expected a request"),
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum ApprovalEvent {
    /// A newly requested authorization.
    Request { request: ApprovalRequest },
    /// A subsequent approval or denial.
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
