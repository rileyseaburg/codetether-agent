//! Serialized provenance envelope for an execution.

use super::AgentIdentity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Execution identifiers, issue time, and the associated agent identity.
///
/// # Examples
/// ```
/// use codetether_provenance::{ExecutionProvenance, ExecutionOrigin};
/// let mut record = ExecutionProvenance::for_operation("worker", ExecutionOrigin::Worker);
/// record.set_run_id("run");
/// assert_eq!(record.run_id.as_deref(),Some("run"));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionProvenance {
    pub provenance_id: String,
    pub session_id: Option<String>,
    pub task_id: Option<String>,
    pub run_id: Option<String>,
    pub attempt_id: Option<String>,
    pub issued_at: DateTime<Utc>,
    pub identity: AgentIdentity,
}
