//! Frontend fields and messages surrounding an approval decision.
/// Native presentation bindings for explicit approval commands.
/// # Implementors
/// Frontends retain status, input, overlay and transcript storage.
pub trait View {
    /// Replace current status text.
    fn set_status(&mut self, status: String);
    /// Read status text when bulk release cannot make progress.
    fn status(&self) -> &str;
    /// Record whether an approval is still pending.
    fn set_waiting(&mut self, waiting: bool);
    /// Reset scrolling for the next approval preview.
    fn reset_preview_scroll(&mut self);
    /// Clear the submitted command.
    fn clear_input(&mut self);
    /// Append a system notice to the transcript.
    fn push_system(&mut self, text: String);
    /// Scroll chat to the notice.
    fn scroll_to_bottom(&mut self);
}
