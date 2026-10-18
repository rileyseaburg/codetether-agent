//! Ctrl+Shift+Y handler: copy the entire conversation transcript.
//!
//! Mouse-selection in the chat panel captures the rendered box
//! borders and trailing whitespace because the terminal does not
//! know about the [`Block`](ratatui::widgets::Block) layout. This
//! handler bypasses that limitation by writing a clean plain-text
//! transcript directly to the system clipboard.

use crate::tui::app::state::App;
use crate::tui::clipboard_text::copy_text;
use crate::tui::constants::SCROLL_BOTTOM;
pub(crate) use codetether_chat_transcript::build_transcript;

pub(super) fn handle_copy_transcript(app: &mut App) {
    let transcript = build_transcript(&app.state.messages);
    if transcript.is_empty() {
        app.state.status = "No conversation to copy yet.".into();
        return;
    }
    let turns = transcript.matches("=== ").count();
    match copy_text(&transcript) {
        Ok(method) => {
            app.state.status = format!("Copied transcript: {turns} turns ({method}).");
            app.state.chat_viewport.chat_scroll = SCROLL_BOTTOM;
        }
        Err(err) => {
            tracing::warn!(error = %err, "Copy transcript to clipboard failed");
            app.state.status = "Could not copy transcript to clipboard.".into();
        }
    }
}
