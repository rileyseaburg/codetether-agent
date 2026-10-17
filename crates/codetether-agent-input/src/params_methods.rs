//! Effective detach flags and trusted resume settings.
use crate::Params;

impl Params {
    /// Returns the effective detach flag, defaulting to synchronous execution.
    pub fn detach_or_default(&self) -> bool {
        self.detach.unwrap_or(false)
    }

    /// Return the explicit spawn detach flag, defaulting to synchronous.
    pub fn detach_for_spawn(&self) -> bool {
        self.detach.unwrap_or(false)
    }

    /// Copy trusted runtime fields into a shared activation configuration.
    pub fn resume_config(&self) -> codetether_agent_lifecycle::ResumeConfig {
        codetether_agent_lifecycle::ResumeConfig::new(
            self._current_model.clone(),
            self.parent_workspace.clone(),
            self.parent_prior_context_allowed,
        )
    }
}
