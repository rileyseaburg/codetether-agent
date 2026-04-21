//! Attachment badge suffix for the input area title.
//!
//! [`attachment_suffix`] produces a short string like `| 📷 2 attached`
//! reflecting pending image attachments only. Mid-stream steering is
//! no longer a concept; see [`crate::tui::app::commands`] for `/ask`.

use crate::tui::app::state::App;

/// Build a short suffix like `| 📷 2 attached` or empty.
///
/// Returns an empty `String` when there are no pending images, so the
/// input title stays clean.
pub fn attachment_suffix(app: &App) -> String {
    let pending_images = app.state.pending_images.len();
    if pending_images == 0 {
        return String::new();
    }
    format!(" | 📷 {pending_images} attached ")
}
