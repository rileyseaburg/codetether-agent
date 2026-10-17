//! Serialized identity attached to an execution.

use super::ExecutionOrigin;
use serde::{Deserialize, Serialize};

/// Execution identity and optional tenant, worker, key, and application identifiers.
///
/// # Examples
/// ```
/// use codetether_provenance::{ExecutionProvenance, ExecutionOrigin};
/// let mut record = ExecutionProvenance::for_operation("worker", ExecutionOrigin::Worker);
/// assert_eq!(record.identity.agent_name,"worker");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentIdentity {
    pub tenant_id: Option<String>,
    pub agent_identity_id: Option<String>,
    pub agent_name: String,
    pub origin: ExecutionOrigin,
    pub worker_id: Option<String>,
    pub key_id: Option<String>,
    pub github_installation_id: Option<String>,
    pub github_app_id: Option<String>,
}
