//! Clipboard paste handler for the TUI.
//!
//! Prefers plain text (routed through the shared chat paste path so
//! image data URLs, sidecar summarisation, and newline handling all
//! match bracketed paste) and tries an image attachment when no text
//! is on the clipboard.
//!
//! # Examples
//!
//! ```ignore
//! handle_clipboard_paste(&mut app);
//! ```

use crate::tui::app::state::App;

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
/// # Examples
///
/// ```ignore
/// handle_clipboard_paste(&mut app);
/// ```
pub(super) fn handle_clipboard_paste(app: &mut App) {
    if let Some(text) = crate::tui::clipboard::get_clipboard_text() {
        let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
        crate::tui::app::input::paste_into_chat(app, &normalized);
        return;
    }

    match crate::tui::clipboard::get_clipboard_image() {
        Some(image) => {
            app.state.pending_images.push(image);
            let n = app.state.pending_images.len();
            app.state.status = if n == 1 {
                "Attached 1 clipboard image. Type a message and press Enter to send.".into()
            } else {
                format!("Attached {n} clipboard images. Press Enter to send them.")
            };
        }
        None => {
            app.state.status = crate::tui::clipboard_ssh::clipboard_unavailable_message();
        }
    }
}
