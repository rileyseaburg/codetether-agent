//! Clipboard image paste handler for the TUI.
//!
//! Extracts an image from the system clipboard and attaches
//! it to the pending images list in [`AppState`].
//!
//! # Examples
//!
//! ```ignore
//! handle_clipboard_paste(&mut app);
//! ```

use crate::tui::app::state::App;

/// Paste an image from the clipboard into pending attachments.
///
/// On success, pushes the image into
/// [`AppState::pending_images`] and updates the status bar.
/// On failure, shows a helpful error suggesting `/image`.
///
/// # Examples
///
/// ```ignore
/// handle_clipboard_paste(&mut app);
/// ```
pub(super) fn handle_clipboard_paste(app: &mut App) {
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
            app.state.status = "No image in clipboard — use /image <path> instead".into();
        }
    }
}
