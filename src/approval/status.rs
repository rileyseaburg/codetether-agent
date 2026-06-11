use serde::{Deserialize, Serialize};

/// Lifecycle state of an approval request.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalStatus {
    /// The request has not been decided yet.
    Pending,
    /// The request was approved and can be verified.
    Approved,
    /// The request was denied and must not be used.
    Denied,
}

impl ApprovalStatus {
    /// Return true when the status represents a granted approval.
    pub fn is_approved(self) -> bool {
        matches!(self, Self::Approved)
    }
}
