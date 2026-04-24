//! Attach pasted image data URLs.

use crate::tui::app::state::App;

/// Attach a pasted image data URL when the text is one.
pub(crate) fn try_attach_data_url(app: &mut App, text: &str) -> bool {
    let Some(image) = crate::image_clipboard::attachment_from_data_url(text) else {
        return false;
    };
    app.state.pending_images.push(image);
    let count = app.state.pending_images.len();
    app.state.status = if count == 1 {
        "Attached pasted image. Type a message and press Enter to send.".to_string()
    } else {
        format!("Attached {count} pasted images. Press Enter to send them.")
    };
    true
}
