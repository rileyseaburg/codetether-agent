//! Editable review-prompt preparation.
/// Standard request used by the review utility command.
pub const PROMPT: &str = "Review the current uncommitted changes for bugs, regressions, security issues, and missing tests. Lead with findings and cite file:line references.";
/// Composer and presentation operations for review preparation.
/// # Implementors
/// Frontends retain input/history mutation and message rendering.
pub trait View {
    /// Clear current input and related composer state.
    fn clear_input(&mut self);
    /// Borrow editable input text.
    fn input(&mut self) -> &mut String;
    /// Borrow the character-based cursor.
    fn cursor(&mut self) -> &mut usize;
    /// Replace status text.
    fn set_status(&mut self, status: String);
    /// Append a system message.
    fn push_system(&mut self, text: String);
    /// Follow the newest message.
    fn scroll_to_bottom(&mut self);
}
/// Populate the review prompt without submitting it.
/// # Arguments
/// Receives the editable composer and presentation operations.
/// # Returns
/// Leaves the cursor at the prompt end and displays the submission hint.
/// # Examples
/// ```no_run
/// fn prepare<V: codetether_slash_command::utility::review::View>(view: &mut V) {
///     codetether_slash_command::utility::review::prepare(view);
/// }
/// ```
pub fn prepare(host: &mut impl View) {
    host.clear_input();
    *host.input() = PROMPT.to_string();
    let cursor = host.input().chars().count();
    *host.cursor() = cursor;
    host.set_status("Review prompt ready".to_string());
    host.push_system("Review prompt is ready. Press Enter to run it.".into());
    host.scroll_to_bottom();
}
