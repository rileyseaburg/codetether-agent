//! Copy utility commands select raw messages rather than rendered lines.
use codetether_chat_model::{ChatMessage, copy};
use codetether_terminal_clipboard::copy_text;
/// Minimal chat state required by the copy command.
/// # Implementors
/// Frontends expose message payloads and composer/status updates.
pub trait View {
    /// Borrow messages in chronological order.
    fn messages(&self) -> &[ChatMessage];
    /// Replace the visible status.
    fn set_status(&mut self, status: String);
    /// Clear the consumed copy command.
    fn clear_input(&mut self);
}
/// Copy the selected reply, tool output or error payload.
/// # Arguments
/// Receives the chat view and raw copy command.
/// # Returns
/// Clears input and reports the selection or clipboard result.
/// # Examples
/// ```no_run
/// fn copy<V: codetether_chat_clipboard::command::View>(view: &mut V) {
///     codetether_chat_clipboard::command::run(view, "/copy reply");
/// }
/// ```
pub fn run(host: &mut impl View, prompt: &str) {
    let Some(target) = copy::target(prompt) else {
        host.set_status("Usage: /copy [reply|tool|error]".into());
        host.clear_input();
        return;
    };
    let status = match copy::latest(host.messages(), target) {
        None => "No matching message to copy.".into(),
        Some(text) => match copy_text(text) {
            Ok(method) => format!("Copied clean text ({method})."),
            Err(error) => {
                tracing::warn!(target: "codetether_agent::tui::app::input::codex_parity_command::copy", error = %error, "Copy message failed");
                format!("Could not copy: {error}")
            }
        },
    };
    host.set_status(status);
    host.clear_input();
}
