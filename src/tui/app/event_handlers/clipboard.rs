//! Clipboard paste handler for the TUI.
//!
//! Prefers plain text (inserted into the chat input as a single block,
//! preserving newlines without submitting them) and falls back to an
//! image attachment when no text is on the clipboard.
//!
//! # Examples
//!
//! ```ignore
//! handle_clipboard_paste(&mut app);
//! ```

use crate::tui::app::state::App;
use crate::tui::models::InputMode;

/// Paste clipboard contents into the chat input.
///
/// Tries text first — this avoids the common failure mode where the
/// terminal does not honor bracketed paste and each newline in the
/// pasted block is delivered as a separate `Enter` key event, which
/// would otherwise submit each line as an independent chat message.
///
/// Falls back to attaching an image from the clipboard when no text
/// is available.
///
/// # Examples
///
/// ```ignore
/// handle_clipboard_paste(&mut app);
/// ```
pub(super) fn handle_clipboard_paste(app: &mut App) {
    if let Some(text) = crate::tui::clipboard::get_clipboard_text() {
        if crate::tui::app::input::try_attach_data_url(app, &text) {
            return;
        }
        insert_clipboard_text(app, &text);
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
            app.state.status =
                "Clipboard unavailable; use /image <path> or `codetether clipboard image` locally."
                    .into();
        }
    }
}

/// Insert clipboard text into the chat input as a single block.
///
/// Mirrors the bracketed-paste handling in
/// [`crate::tui::app::input::handle_paste`] so that multi-line clipboard
/// contents stay in the input buffer instead of each line submitting
/// itself as a separate chat message.
fn insert_clipboard_text(app: &mut App, text: &str) {
    let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
    app.state.input_mode = if app.state.input.is_empty() && normalized.starts_with('/') {
        InputMode::Command
    } else if app.state.input.starts_with('/') {
        InputMode::Command
    } else {
        InputMode::Editing
    };
    app.state.insert_text(&normalized);
    let line_count = normalized.lines().count();
    app.state.status = if line_count > 1 {
        format!("Pasted {line_count} lines from clipboard")
    } else {
        "Pasted from clipboard".to_string()
    };
}
