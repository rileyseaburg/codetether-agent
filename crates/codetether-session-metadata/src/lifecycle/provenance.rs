use crate::SessionMetadata;
use codetether_provenance::ClaimProvenance;
/// Set the owning agent and update existing provenance.
/// # Arguments
/// `agent` is the session persona; `metadata` owns optional provenance;
/// `agent_name` is the replacement name.
/// # Returns
/// Updates both records in place without creating missing provenance.
/// # Examples
/// ```
/// let mut agent = String::new();
/// let mut metadata = codetether_session_metadata::SessionMetadata::default();
/// codetether_session_metadata::lifecycle::set_agent_name(&mut agent, &mut metadata, "build");
/// assert_eq!(agent, "build");
/// ```
pub fn set_agent_name(
    agent: &mut String,
    metadata: &mut SessionMetadata,
    agent_name: impl Into<String>,
) {
    let agent_name = agent_name.into();
    *agent = agent_name.clone();
    if let Some(provenance) = metadata.provenance.as_mut() {
        provenance.set_agent_name(&agent_name);
    }
}
/// Attach worker and task identifiers to existing provenance.
/// # Arguments
/// `metadata` owns provenance; `worker_id` and `task_id` identify dispatch.
/// # Returns
/// Updates existing provenance in place, or does nothing when it is absent.
/// # Examples
/// ```
/// let mut metadata = codetether_session_metadata::SessionMetadata::default();
/// codetether_session_metadata::lifecycle::attach_worker_task(&mut metadata, "worker", "task");
/// ```
pub fn attach_worker_task(metadata: &mut SessionMetadata, worker_id: &str, task_id: &str) {
    if let Some(provenance) = metadata.provenance.as_mut() {
        provenance.apply_worker_task(worker_id, task_id);
    }
}
/// Attach an accepted task claim to existing provenance.
/// # Arguments
/// `metadata` owns provenance; `claim` supplies the accepted claim record.
/// # Returns
/// Updates existing provenance in place, or does nothing when it is absent.
/// # Examples
/// ```ignore
/// attach_claim(&mut metadata, &accepted_claim);
/// ```
pub fn attach_claim(metadata: &mut SessionMetadata, claim: &ClaimProvenance) {
    if let Some(provenance) = metadata.provenance.as_mut() {
        provenance.apply_claim(claim);
    }
}
