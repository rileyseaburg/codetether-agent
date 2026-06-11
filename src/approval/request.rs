use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Approval request for one tool/action/resource tuple.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApprovalRequest {
    /// Stable id passed back as `approval_id`.
    pub id: String,
    /// Tool id, for example `apply_patch`.
    pub tool: String,
    /// Requested action, for example `write`.
    pub action: String,
    /// Resource being affected.
    pub resource: String,
    /// Human-readable reason for the request.
    pub reason: String,
    /// UTC timestamp when the request was created.
    pub requested_at: DateTime<Utc>,
}

impl ApprovalRequest {
    /// Create a new pending approval request.
    pub fn new(tool: &str, action: &str, resource: &str, reason: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            tool: tool.to_string(),
            action: action.to_string(),
            resource: resource.to_string(),
            reason: reason.to_string(),
            requested_at: Utc::now(),
        }
    }

    pub(crate) fn matches(&self, tool: &str, action: &str, resource: &str) -> bool {
        self.tool == tool && self.action == action && self.resource == resource
    }
}
