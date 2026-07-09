//! Recover image data URLs that chunked pastes diverted into the
//! large-paste text sidecar.
//!
//! tmux and some terminals split one oversized paste into several
//! paste events. Each fragment of the base64 payload fails image
//! decoding on its own and lands in `pending_text_pastes` as a
//! `[Pasted text #N: …]` placeholder. At submit time this module
//! expands those placeholders back into the input verbatim,
//! reassembling the original data URL, and promotes it to an image.

use crate::tui::app::state::App;

use super::image_data_paste::{drain_embedded_images, extract_image_data_url};
use super::paste_expand_raw::expand_placeholders_raw;

/// Drain inline data URLs, then recover any split across sidecar pastes.
pub(crate) fn recover_pasted_images(app: &mut App) -> usize {
    drain_embedded_images(app) + recover_sidecar_images(app)
}

/// Expand sidecar placeholders verbatim, extract reassembled image
/// data URLs, and keep only the remaining text inline.
fn recover_sidecar_images(app: &mut App) -> usize {
    if app.state.pending_text_pastes.is_empty() {
        return 0;
    }
    let expanded = expand_placeholders_raw(&app.state.input, &app.state.pending_text_pastes);
    if !expanded.contains("data:image/") {
        return 0;
    }
    let mut text = expanded;
    let mut attached = 0;
    while let Some(found) = extract_image_data_url(&text) {
        let Some(image) = crate::image_clipboard::attachment_from_data_url(&found.data_url) else {
            break;
        };
        app.state.pending_images.push(image);
        text = found.remainder;
        attached += 1;
    }
    if attached > 0 {
        app.state.pending_text_pastes.clear();
        app.state.input = text;
        app.state.input_cursor = app.state.input.chars().count();
        let total = app.state.pending_images.len();
        app.state.status = format!("Recovered {attached} pasted image(s); {total} attached.");
    }
    attached
}
