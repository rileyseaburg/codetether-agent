//! Ctrl+Shift+Y handler: copy the entire conversation transcript.
//!
//! Mouse-selection in the chat panel captures the rendered box
//! borders and trailing whitespace because the terminal does not
//! know about the [`Block`](ratatui::widgets::Block) layout. This
//! handler bypasses that limitation by writing a clean plain-text
//! transcript directly to the system clipboard.

use crate::Host;
use codetether_terminal_clipboard::copy_text;

use codetether_chat_transcript::build_transcript;

/// Copy the full plain-text conversation transcript.
/// # Arguments
/// * `host` — Chat state and frontend effects.
/// # Returns
/// Updates frontend state to reflect the clipboard operation.
/// # Examples
/// ```no_run
/// # fn example(host: &mut impl codetether_chat_clipboard::Host) {
/// codetether_chat_clipboard::handle_copy_transcript(host);
/// # }
/// ```
pub fn handle_copy_transcript(host: &mut impl Host) {
    let transcript = build_transcript(host.messages());
    if transcript.is_empty() {
        host.set_status("No conversation to copy yet.".into());
        return;
    }
    let turns = transcript.matches("=== ").count();
    match copy_text(&transcript) {
        Ok(method) => {
            host.set_status(format!("Copied transcript: {turns} turns ({method})."));
            host.scroll_bottom();
        }
        Err(err) => {
            tracing::warn!(target: "codetether_agent::tui::app::event_handlers::copy_transcript", error = %err, "Copy transcript to clipboard failed");
            host.set_status("Could not copy transcript to clipboard.".into());
        }
    }
}
