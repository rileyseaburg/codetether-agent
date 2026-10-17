//! Serialized ApprovalDecision data.

use crate::ApprovalChoice;
use crate::defaults::utc_now;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Approval decision for an OKR run
///
/// # Examples
/// ```
/// use codetether_okr::{ApprovalDecision, ApprovalChoice};
/// let value = ApprovalDecision::approve(uuid::Uuid::nil(), "Reviewed");
/// assert_eq!(value.decision, ApprovalChoice::Approved);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalDecision {
    /// Unique identifier for this decision
    pub id: Uuid,

    /// ID of the run this decision applies to
    pub run_id: Uuid,

    /// The actual decision
    pub decision: ApprovalChoice,

    /// Reason for the decision
    #[serde(default)]
    pub reason: String,

    /// Who made the decision
    #[serde(default)]
    pub approver: Option<String>,

    /// Additional context/metadata
    #[serde(default)]
    pub metadata: std::collections::HashMap<String, String>,

    /// Timestamp of the decision
    #[serde(default = "utc_now")]
    pub decided_at: DateTime<Utc>,
}
