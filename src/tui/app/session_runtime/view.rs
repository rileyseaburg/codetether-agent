//! Lightweight render view for a checked-out session.

use std::path::PathBuf;

use crate::session::Session;

/// Read-only session facts needed while the full session is moved.
///
/// # Examples
///
/// ```
/// use codetether_agent::tui::app::session_runtime::SessionView;
///
/// let view = SessionView::default();
/// assert_eq!(view.message_count, 0);
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SessionView {
    /// Active session id.
    pub id: String,
    /// Selected model, if one has been pinned.
    pub model: Option<String>,
    /// Workspace directory associated with the session.
    pub directory: Option<PathBuf>,
    /// Whether edit confirmations are auto-applied.
    pub auto_apply_edits: bool,
    /// Whether network-reaching tools are allowed.
    pub allow_network: bool,
    /// Whether slash-command autocomplete is enabled.
    pub slash_autocomplete: bool,
    /// Whether prompt execution uses isolated git worktrees.
    pub use_worktree: bool,
    /// Number of transcript messages in the current session.
    pub message_count: usize,
}

impl SessionView {
    /// Capture the current display-safe session state.
    pub(crate) fn from_session(session: &Session) -> Self {
        Self {
            id: session.id.clone(),
            model: session.metadata.model.clone(),
            directory: session.metadata.directory.clone(),
            auto_apply_edits: session.metadata.auto_apply_edits,
            allow_network: session.metadata.allow_network,
            slash_autocomplete: session.metadata.slash_autocomplete,
            use_worktree: session.metadata.use_worktree,
            message_count: session.messages.len(),
        }
    }

    /// Return the active session id if one is known.
    pub(crate) fn id(&self) -> &str {
        &self.id
    }
}
