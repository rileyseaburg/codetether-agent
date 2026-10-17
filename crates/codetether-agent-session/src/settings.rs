//! Borrowed fields used to initialize and refresh managed child sessions.
use std::path::PathBuf;
/// Mutable access to the session settings owned by the application.
/// # Examples
/// ```rust
/// use codetether_agent_session::Settings;
/// let (mut model, mut directory, mut allowed, mut edits) = (None, None, None, false);
/// let settings = Settings { model: &mut model, directory: &mut directory,
///     inherited_prior_context_allowed: &mut allowed, auto_apply_edits: &mut edits,
///     workspace_pinned: false };
/// *settings.auto_apply_edits = true;
/// assert!(edits);
/// ```
pub struct Settings<'a> {
    /// Existing model override, updated in place.
    pub model: &'a mut Option<String>,
    /// Existing workspace directory, updated in place.
    pub directory: &'a mut Option<PathBuf>,
    /// Existing inherited access ceiling, updated in place.
    pub inherited_prior_context_allowed: &'a mut Option<bool>,
    /// Whether task edits apply without an interactive preview.
    pub auto_apply_edits: &'a mut bool,
    /// Whether a task-owned workspace must survive parent refreshes.
    pub workspace_pinned: bool,
}
