use super::{State, sidecar::recover_sidecar_images};
/// Drain inline images, then recover images split across sidecar pastes.
/// # Arguments
/// `state` borrows the composer fields and text sidecars.
/// # Returns
/// Total images attached, retaining sidecars when none could be recovered.
/// # Examples
/// ```
/// let (mut images, mut input, mut cursor, mut status, mut pastes) =
///     (Vec::new(), String::new(), 0, String::new(), Vec::new());
/// let mut state = codetether_paste_buffer::image_recovery::State {
///     composer: codetether_image::paste::State {
///         images: &mut images, input: &mut input, cursor: &mut cursor, status: &mut status,
///     }, pastes: &mut pastes,
/// };
/// assert_eq!(codetether_paste_buffer::image_recovery::recover_pasted_images(&mut state), 0);
/// ```
pub fn recover_pasted_images(state: &mut State<'_>) -> usize {
    codetether_image::paste::drain_embedded_images(&mut state.composer)
        + recover_sidecar_images(state)
}
