//! Clipboard paste handler for the TUI.
//!
//! Prefers plain text (routed through the shared chat paste path so
//! image data URLs, sidecar summarisation, and newline handling all
//! match bracketed paste) and tries an image attachment when no text
//! is on the clipboard.
//!
//! # Examples
//!
//! ```text
//! handle_clipboard_paste(host);
//! ```

use crate::Host;

/// Paste clipboard contents into the chat input.
///
/// Tries text first — this avoids the common failure mode where the
/// terminal does not honor bracketed paste and each newline in the
/// pasted block is delivered as a separate `Enter` key event, which
/// would otherwise submit each line as an independent chat message.
///
/// When no text is available, attaches an image from the clipboard;
/// when neither is available, shows SSH bridge guidance.
///

/// Read text or an image from the terminal clipboard.
/// # Arguments
/// * `host` — Chat state and frontend effects.
/// # Returns
/// Updates frontend state to reflect the clipboard operation.
/// # Examples
/// ```no_run
/// # fn example(host: &mut impl codetether_chat_clipboard::Host) {
/// codetether_chat_clipboard::handle_clipboard_paste(host);
/// # }
/// ```
pub fn handle_clipboard_paste(host: &mut impl Host) {
    if let Some(text) = codetether_terminal_clipboard::get_clipboard_text() {
        let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
        host.paste_into_chat(&normalized);
        return;
    }

    match codetether_terminal_clipboard::get_clipboard_image() {
        Some(image) => {
            host.pending_images().push(image);
            let n = host.pending_images().len();
            host.set_status(if n == 1 {
                "Attached 1 clipboard image. Type a message and press Enter to send.".into()
            } else {
                format!("Attached {n} clipboard images. Press Enter to send them.")
            });
        }
        None => {
            host.set_status(codetether_terminal_clipboard::clipboard_unavailable_message());
        }
    }
}
