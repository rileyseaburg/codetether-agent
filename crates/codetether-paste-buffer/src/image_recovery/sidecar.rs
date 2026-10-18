//! Reassemble placeholders and promote valid data URLs to images.
use super::State;
pub(super) fn recover_sidecar_images(state: &mut State<'_>) -> usize {
    if state.pastes.is_empty() {
        return 0;
    }
    let expanded = crate::expand_placeholders_raw(state.composer.input.as_str(), state.pastes);
    if !expanded.contains("data:image/") {
        return 0;
    }
    let mut text = expanded;
    let mut attached = 0;
    while let Some(found) = codetether_image::extract_image_data_url(&text) {
        let Some(image) = codetether_image::attachment_from_data_url(&found.data_url) else {
            break;
        };
        state.composer.images.push(image);
        text = found.remainder;
        attached += 1;
    }
    if attached > 0 {
        state.pastes.clear();
        *state.composer.input = text;
        *state.composer.cursor = state.composer.input.chars().count();
        let total = state.composer.images.len();
        *state.composer.status = format!("Recovered {attached} pasted image(s); {total} attached.");
    }
    attached
}
