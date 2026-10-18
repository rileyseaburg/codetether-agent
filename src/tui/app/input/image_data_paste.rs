//! Attach pasted image data URLs, including URLs embedded in larger text.

use crate::tui::app::state::App;

#[path = "image_data_extract.rs"]
mod extract;
pub(crate) use extract::extract_image_data_url;

#[cfg(test)]
#[path = "image_data_paste_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "image_data_reject_tests.rs"]
mod reject_tests;

/// Handle a paste that is entirely one image data URL.
///
/// # Returns
///
/// `true` when the paste was consumed — either attached as an image, or
/// recognised as an image that cannot be attached (for example a screenshot
/// over the size cap) and reported to the user. Consuming a rejected image
/// keeps megabytes of base64 out of the large-paste text sidecar, where the
/// failure would otherwise be silent.
pub(crate) fn try_attach_data_url(app: &mut App, text: &str) -> bool {
    codetether_image::paste::try_attach_data_url(&mut state(app), text)
}

/// Pull every embedded image data URL out of `app.state.input`, attach each,
/// and replace the input with the remaining text. Returns the count attached.
pub(crate) fn drain_embedded_images(app: &mut App) -> usize {
    codetether_image::paste::drain_embedded_images(&mut state(app))
}

fn state(app: &mut App) -> codetether_image::paste::State<'_> {
    codetether_image::paste::State {
        images: &mut app.state.pending_images,
        input: &mut app.state.input,
        cursor: &mut app.state.input_cursor,
        status: &mut app.state.status,
    }
}
