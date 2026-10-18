//! Read-only session facts retained during runtime ownership handoff.
use std::path::PathBuf;

/// Read-only session facts needed while the full session is moved.
///
/// # Examples
///
/// ```
/// use codetether_tui_models::SessionView;
///
/// let view = SessionView::<()>::default();
/// assert_eq!(view.message_count, 0);
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SessionView<P> {
    /// Active session id.
    pub id: String,
    /// Human-readable session title, when available.
    pub title: Option<String>,
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
    /// Identity and provenance displayed as metadata outside the transcript.
    pub principal: P,
}
impl<P> SessionView<P> {
    /// Return the active session id if one is known.
    pub fn id(&self) -> &str {
        &self.id
    }
}
