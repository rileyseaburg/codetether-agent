//! RAII cleanup for one active child-agent turn.

/// Guard representing the active message run for one sub-agent.
pub(in crate::tool::agent) struct AgentRunGuard {
    pub(super) name: String,
}

impl Drop for AgentRunGuard {
    fn drop(&mut self) {
        super::registry::finish(&self.name);
    }
}
