//! ApprovalDecision decision behavior.

use crate::ApprovalChoice;
use crate::ApprovalDecision;
use chrono::Utc;
use uuid::Uuid;
impl ApprovalDecision {
    /// Create an approval
    ///
    /// # Examples
    /// ```
    /// use codetether_okr::{ApprovalDecision, ApprovalChoice};
    /// let value = ApprovalDecision::approve(uuid::Uuid::nil(), "Reviewed");
    /// assert_eq!(value.decision, ApprovalChoice::Approved);
    /// ```
    pub fn approve(run_id: Uuid, reason: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            run_id,
            decision: ApprovalChoice::Approved,
            reason: reason.into(),
            approver: None,
            metadata: std::collections::HashMap::new(),
            decided_at: Utc::now(),
        }
    }

    /// Create a denial
    ///
    /// # Examples
    /// ```
    /// use codetether_okr::{ApprovalDecision, ApprovalChoice};
    /// let value = ApprovalDecision::deny(uuid::Uuid::nil(), "Needs review");
    /// assert_eq!(value.decision, ApprovalChoice::Denied);
    /// ```
    pub fn deny(run_id: Uuid, reason: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            run_id,
            decision: ApprovalChoice::Denied,
            reason: reason.into(),
            approver: None,
            metadata: std::collections::HashMap::new(),
            decided_at: Utc::now(),
        }
    }
}
