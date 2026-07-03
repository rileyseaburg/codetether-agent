//! Attach pasted image data URLs, including URLs embedded in larger text.

use crate::tui::app::state::App;

#[path = "image_data_extract.rs"]
mod extract;
pub(crate) use extract::extract_image_data_url;

#[cfg(test)]
#[path = "image_data_paste_tests.rs"]
mod tests;

/// Attach a pasted image data URL when the whole text is one.
pub(crate) fn try_attach_data_url(app: &mut App, text: &str) -> bool {
    let Some(image) = crate::image_clipboard::attachment_from_data_url(text) else {
        return false;
    };
    app.state.pending_images.push(image);
    let count = app.state.pending_images.len();
    app.state.status = if count == 1 {
        "Attached pasted image. Press Enter to send, or add text first.".to_string()
    } else {
        format!("Attached {count} pasted images. Press Enter to send, or add text first.")
    };
    true
}

/// Pull every embedded image data URL out of `app.state.input`, attach each,
/// and replace the input with the remaining text. Returns the count attached.
pub(crate) fn drain_embedded_images(app: &mut App) -> usize {
    let mut attached = 0;
    while let Some(found) = extract_image_data_url(&app.state.input) {
        let Some(image) = crate::image_clipboard::attachment_from_data_url(&found.data_url) else {
            break;
        };
        app.state.pending_images.push(image);
        app.state.input = found.remainder;
        app.state.input_cursor = app.state.input.chars().count();
        attached += 1;
    }
    if attached > 0 {
        let total = app.state.pending_images.len();
        app.state.status = format!("Attached {total} pasted image(s) from clipboard.");
    }
    attached
}
