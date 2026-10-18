//! Native session metadata and frontend hydration bindings.
use codetether_workspace_snapshot::WorkspaceSnapshot;
/// Session metadata consumed while hydrating a frontend.
/// # Implementors
/// Applications provide their current session without changing its ownership.
pub trait Session {
    /// Current session identifier.
    fn id(&self) -> &str;
    /// Whether automatic edits are enabled.
    fn auto_apply(&self) -> bool;
    /// Whether networking is enabled by session metadata.
    fn allow_network(&self) -> bool;
    /// Whether slash completion is enabled.
    fn autocomplete(&self) -> bool;
    /// Whether worktree execution is enabled.
    fn use_worktree(&self) -> bool;
    /// Apply the effective startup network setting to session metadata.
    fn set_allow_network(&mut self, enabled: bool);
}
/// Frontend fields populated by startup hydration.
/// # Implementors
/// Native frontends bind their existing scalar and workspace fields.
pub trait Fields {
    /// Install the loaded workspace snapshot.
    fn set_workspace(&mut self, workspace: WorkspaceSnapshot);
    /// Set automatic-edit policy.
    fn set_auto_apply(&mut self, enabled: bool);
    /// Set network policy.
    fn set_allow_network(&mut self, enabled: bool);
    /// Set slash-completion policy.
    fn set_autocomplete(&mut self, enabled: bool);
    /// Set worktree execution policy.
    fn set_use_worktree(&mut self, enabled: bool);
    /// Set the current session identity.
    fn set_session_id(&mut self, id: String);
    /// Set the displayed workspace directory.
    fn set_cwd(&mut self, cwd: String);
    /// Set whether the peer endpoint is ready.
    fn set_peer_ready(&mut self, ready: bool);
    /// Replace the frontend status text.
    fn set_status(&mut self, status: String);
}
/// Native services and presentation surrounding hydrated fields.
/// # Implementors
/// Frontends preserve transcript sync, worker registration and input behavior.
pub trait Effects<S: Session>: Fields {
    /// Application worker bridge.
    type Worker;
    /// Synchronize visible history from the selected source.
    fn sync_messages(&mut self, session: &S, source: &str, has_older: bool);
    /// Register an available worker bridge.
    fn attach_worker(&mut self, worker: Option<&Self::Worker>);
    /// Refresh command suggestions.
    fn refresh_suggestions(&mut self);
    /// Place the input cursor at the end.
    fn move_cursor_end(&mut self);
    /// Append a startup notice.
    fn push_system(&mut self, text: String);
    /// Scroll chat to the notice.
    fn scroll_to_bottom(&mut self);
    /// Current startup trust-policy description.
    fn policy_summary(&self) -> Option<String>;
}
/// Complete frontend startup bindings.
/// # Implementors
/// Automatically implemented for Fields + Effects.
pub trait Host<S: Session>: Effects<S> {}
impl<S: Session, T: Effects<S>> Host<S> for T {}
