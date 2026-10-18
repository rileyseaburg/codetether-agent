//! Read-only report fields and presentation operations.
/// Current session and TUI values used by a status report.
/// # Examples
/// ```
/// let snapshot = codetether_chat_status::command::Snapshot {
///     session_id: "session", model: None, auto_apply_edits: false,
///     allow_network: false, use_worktree: true, slash_autocomplete: true,
///     context_used: None, context_budget: None, last_completion_model: None,
///     last_completion_prompt_tokens: None, last_completion_output_tokens: None,
/// };
/// assert_eq!(snapshot.session_id, "session");
/// ```
pub struct Snapshot<'a> {
    /// Durable session identity.
    pub session_id: &'a str,
    /// Configured model selector, if any.
    pub model: Option<&'a str>,
    /// Whether edits auto-apply.
    pub auto_apply_edits: bool,
    /// Whether network access is enabled.
    pub allow_network: bool,
    /// Whether isolated worktrees are enabled.
    pub use_worktree: bool,
    /// Whether slash completion is enabled.
    pub slash_autocomplete: bool,
    /// Current estimated context tokens.
    pub context_used: Option<usize>,
    /// Current context token budget.
    pub context_budget: Option<usize>,
    /// Most recent completed model.
    pub last_completion_model: Option<&'a str>,
    /// Most recent prompt token count.
    pub last_completion_prompt_tokens: Option<usize>,
    /// Most recent output token count.
    pub last_completion_output_tokens: Option<usize>,
}
/// Native state projection and report presentation.
/// # Implementors
/// Frontends provide borrowed report fields and existing transcript operations.
pub trait View {
    /// Read current report fields after trust policy refresh.
    fn snapshot(&self) -> Snapshot<'_>;
    /// Append the report as a system message.
    fn push_system(&mut self, text: String);
    /// Follow the newest report.
    fn scroll_to_bottom(&mut self);
    /// Clear consumed command input.
    fn clear_input(&mut self);
    /// Replace the visible command status.
    fn set_status(&mut self, status: String);
}
