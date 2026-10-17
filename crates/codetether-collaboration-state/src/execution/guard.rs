//! RAII cleanup for one active child-agent turn.

/// Guard representing the active message run for one sub-agent.
///
/// Dropping it clears the running flag, removes the registered abort handle,
/// and signals completion after releasing the execution registry lock.
///
/// # Examples
/// ```
/// use codetether_collaboration_state::execution::{try_start,is_running};
/// let guard=try_start("docs-guard").unwrap();
/// assert!(is_running("docs-guard"));
/// drop(guard);
/// assert!(!is_running("docs-guard"));
/// ```
pub struct AgentRunGuard {
    pub(super) name: String,
}

impl Drop for AgentRunGuard {
    fn drop(&mut self) {
        super::registry::finish(&self.name);
    }
}
