//! Ctrl+Y handler: copy latest assistant reply to clipboard.

use crate::Host;
use codetether_chat_model::MessageType;
use codetether_terminal_clipboard::copy_text;

use codetether_chat_transcript::clipboard_text;

/// Copy the most recent nonempty assistant reply.
/// # Arguments
/// * `host` — Chat state and frontend effects.
/// # Returns
/// Updates frontend state to reflect the clipboard operation.
/// # Examples
/// ```no_run
/// # fn example(host: &mut impl codetether_chat_clipboard::Host) {
/// codetether_chat_clipboard::handle_copy_reply(host);
/// # }
/// ```
pub fn handle_copy_reply(host: &mut impl Host) {
    let msg =
        host.messages().iter().rev().find(|m| {
            matches!(m.message_type, MessageType::Assistant) && !m.content.trim().is_empty()
        });
    let Some(msg) = msg else {
        host.set_status("Nothing to copy yet.".into());
        return;
    };
    let text = clipboard_text(msg);
    match copy_text(&text) {
        Ok(method) => {
            host.set_status(format!("Copied latest reply ({method})."));
            host.scroll_bottom();
        }
        Err(err) => {
            tracing::warn!(target: "codetether_agent::tui::app::event_handlers::copy_reply", error = %err, "Copy to clipboard failed");
            host.set_status("Could not copy to clipboard.".into());
        }
    }
}
