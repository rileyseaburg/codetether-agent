use super::ApprovalStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Recorded decision for an approval request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApprovalDecision {
    /// Unique decision event id.
    pub id: String,
    /// Request id this decision applies to.
    pub request_id: String,
    /// Approved or denied status.
    pub status: ApprovalStatus,
    /// Actor that made the decision.
    pub decided_by: String,
    /// Human-readable decision reason.
    pub reason: String,
    /// UTC timestamp when the decision was recorded.
    pub decided_at: DateTime<Utc>,
}

impl ApprovalDecision {
    /// Create an approval decision.
    pub fn approve(request_id: &str, decided_by: &str, reason: &str) -> Self {
        Self::new(request_id, ApprovalStatus::Approved, decided_by, reason)
    }

    /// Create a denial decision.
    pub fn deny(request_id: &str, decided_by: &str, reason: &str) -> Self {
        Self::new(request_id, ApprovalStatus::Denied, decided_by, reason)
    }

    fn new(request_id: &str, status: ApprovalStatus, decided_by: &str, reason: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            request_id: request_id.to_string(),
            status,
            decided_by: decided_by.to_string(),
            reason: reason.to_string(),
            decided_at: Utc::now(),
        }
    }
}
