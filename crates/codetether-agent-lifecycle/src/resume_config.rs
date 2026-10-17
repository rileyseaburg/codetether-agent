//! Live parent configuration applied when reopening a durable child.

use std::path::PathBuf;

/// Runtime-owned settings available to a resumed child.
/// # Examples
/// ```rust
/// use codetether_agent_lifecycle::ResumeConfig;
/// let config = ResumeConfig::new(Some("provider/model".into()), None, Some(false));
/// assert!(!config.is_empty());
/// ```
#[derive(Clone, Default)]
pub struct ResumeConfig {
    /// Model override from the current parent.
    pub model: Option<String>,
    /// Workspace override from the current parent.
    pub workspace: Option<PathBuf>,
    /// Whether inherited context is allowed by the current parent.
    pub prior_context_allowed: Option<bool>,
}

impl ResumeConfig {
    /// Construct a configuration snapshot from trusted tool runtime fields.
    /// # Arguments
    /// * `model` — Optional model override.
    /// * `workspace` — Optional workspace override.
    /// * `prior_context_allowed` — Optional inherited-context policy.
    /// # Returns
    /// A snapshot retaining all three optional overrides.
    /// # Examples
    /// See [ResumeConfig].
    pub fn new(
        model: Option<String>,
        workspace: Option<PathBuf>,
        prior_context_allowed: Option<bool>,
    ) -> Self {
        Self {
            model,
            workspace,
            prior_context_allowed,
        }
    }

    /// Return whether no live override is present.
    pub fn is_empty(&self) -> bool {
        self.model.is_none() && self.workspace.is_none() && self.prior_context_allowed.is_none()
    }
}
