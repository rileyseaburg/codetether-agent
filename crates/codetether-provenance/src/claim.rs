//! Worker task-claim metadata and its application to execution provenance.
//!
//! Build a [ClaimProvenance] and pass it to [crate::ExecutionProvenance::apply_claim].
mod apply;
/// Metadata from a worker's task claim, applied without erasing known identity.
///
/// # Examples
/// ```
/// use codetether_provenance::ClaimProvenance;
/// let claim = ClaimProvenance { worker_id: "worker".into(), ..Default::default() };
/// assert_eq!(claim.worker_id, "worker");
/// ```
#[derive(Debug, Clone, Default)]
pub struct ClaimProvenance {
    pub worker_id: String,
    pub task_id: String,
    pub run_id: Option<String>,
    pub attempt_id: Option<String>,
    pub tenant_id: Option<String>,
    pub agent_identity_id: Option<String>,
}
