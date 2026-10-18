//! Interactive preferences and view updates used by session replacement.
use std::{future::Future, path::Path};
/// Boolean preferences inherited from the current interactive view.
/// # Examples
/// ```
/// let preferences = codetether_new_session::Preferences {
///     auto_apply_edits: false, allow_network: false,
///     slash_autocomplete: true, use_worktree: true,
/// };
/// assert!(preferences.use_worktree);
/// ```
pub struct Preferences {
    /// Whether edits are automatically applied.
    pub auto_apply_edits: bool,
    /// Whether network access is enabled.
    pub allow_network: bool,
    /// Whether slash autocomplete is enabled.
    pub slash_autocomplete: bool,
    /// Whether worktree isolation is enabled.
    pub use_worktree: bool,
}
/// View mutations and session-list refresh for a new conversation.
/// # Implementors
/// Native clients map these methods to existing application state operations.
pub trait View {
    /// Read the current interactive preferences.
    fn preferences(&self) -> Preferences;
    /// Set status for creation or persistence results.
    fn set_status(&mut self, status: String);
    /// Set the new active session identity.
    fn set_session_id(&mut self, id: String);
    /// Clear displayed conversation messages.
    fn clear_messages(&mut self);
    /// Clear retained streaming text.
    fn clear_streaming_text(&mut self);
    /// Reset the processing flag to false.
    fn clear_processing(&mut self);
    /// Clear request timing.
    fn clear_request_timing(&mut self);
    /// Scroll to the new conversation's end.
    fn scroll_to_bottom(&mut self);
    /// Select chat view.
    fn show_chat(&mut self);
    /// Refresh the workspace session list.
    fn refresh_sessions(&mut self, cwd: &Path) -> impl Future<Output = ()>;
}
