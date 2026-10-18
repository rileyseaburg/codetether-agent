use super::State;

/// Pull every embedded image data URL out of the composer input, attach each,
/// and replace the input with the remaining text. Returns the count attached.
/// # Arguments
/// `state` borrows composer text, attachments, cursor and status.
/// # Returns
/// Number of images attached before input exhaustion or an invalid image.
/// # Examples
/// ```
/// let (mut images, mut input, mut cursor, mut status) = (Vec::new(), "plain text".to_string(), 0, String::new());
/// let mut state = codetether_image::paste::State {
///     images: &mut images, input: &mut input, cursor: &mut cursor, status: &mut status,
/// };
/// assert_eq!(codetether_image::paste::drain_embedded_images(&mut state), 0);
/// assert_eq!(state.input.as_str(), "plain text");
/// ```
pub fn drain_embedded_images(state: &mut State<'_>) -> usize {
    let mut attached = 0;
    while let Some(found) = crate::extract_image_data_url(state.input.as_str()) {
        let Some(image) = crate::attachment_from_data_url(&found.data_url) else {
            break;
        };
        state.images.push(image);
        *state.input = found.remainder;
        *state.cursor = state.input.chars().count();
        attached += 1;
    }
    if attached > 0 {
        let total = state.images.len();
        *state.status = format!("Attached {total} pasted image(s) from clipboard.");
    }
    attached
}
