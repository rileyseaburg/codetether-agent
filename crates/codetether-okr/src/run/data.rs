//! Serialized OkrRun data.

use crate::defaults::utc_now;
use crate::{ApprovalDecision, KrOutcome, OkrRunStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// An execution run of an OKR (multiple runs per OKR are allowed)
///
/// # Examples
/// ```
/// use codetether_okr::{OkrRun};
/// let mut value = OkrRun::new(uuid::Uuid::nil(), "Quarterly run");
/// assert_eq!(value.name, "Quarterly run");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OkrRun {
    /// Unique identifier for this run
    pub id: Uuid,

    /// Parent OKR ID
    pub okr_id: Uuid,

    /// Human-readable name for this run
    pub name: String,

    /// Current status of the run
    #[serde(default)]
    pub status: OkrRunStatus,

    /// Correlation ID linking to relay/session
    #[serde(default)]
    pub correlation_id: Option<String>,

    /// Relay checkpoint ID for resume capability
    #[serde(default)]
    pub relay_checkpoint_id: Option<String>,

    /// Session ID if applicable
    #[serde(default)]
    pub session_id: Option<String>,

    /// Progress per key result (kr_id -> progress)
    #[serde(default)]
    pub kr_progress: std::collections::HashMap<String, f64>,

    /// Approval decision for this run
    #[serde(default)]
    pub approval: Option<ApprovalDecision>,

    /// List of outcomes achieved in this run
    #[serde(default)]
    pub outcomes: Vec<KrOutcome>,

    /// Iteration count for this run
    #[serde(default)]
    pub iterations: u32,

    /// Started timestamp
    #[serde(default = "utc_now")]
    pub started_at: DateTime<Utc>,

    /// Completed timestamp (if finished)
    #[serde(default)]
    pub completed_at: Option<DateTime<Utc>>,

    /// Last update timestamp
    #[serde(default = "utc_now")]
    pub updated_at: DateTime<Utc>,
}
