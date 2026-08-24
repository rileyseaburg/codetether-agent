//! Live parent configuration applied when reopening a durable child.

use std::path::PathBuf;

/// Runtime-owned settings available to a resumed child.
#[derive(Clone, Default)]
pub(in crate::tool) struct ResumeConfig {
    pub(super) model: Option<String>,
    pub(super) workspace: Option<PathBuf>,
    pub(super) prior_context_allowed: Option<bool>,
    pub(in crate::tool) network_allowed: Option<bool>,
}

impl ResumeConfig {
    /// Construct a configuration snapshot from trusted tool runtime fields.
    pub(in crate::tool) fn new(
        model: Option<String>,
        workspace: Option<PathBuf>,
        prior_context_allowed: Option<bool>,
    ) -> Self {
        Self {
            model,
            workspace,
            prior_context_allowed,
            network_allowed: None,
        }
    }

    pub(in crate::tool) fn with_network(mut self, allowed: Option<bool>) -> Self {
        self.network_allowed = allowed;
        self
    }

    pub(super) fn is_empty(&self) -> bool {
        self.model.is_none()
            && self.workspace.is_none()
            && self.prior_context_allowed.is_none()
            && self.network_allowed.is_none()
    }
}
