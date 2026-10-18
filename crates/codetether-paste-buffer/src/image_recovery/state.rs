use crate::PendingTextPaste;
/// Borrowed composer and sidecars used for image recovery.
/// # Examples
/// ```
/// let (mut images, mut input, mut cursor, mut status, mut pastes) =
///     (Vec::new(), String::new(), 0, String::new(), Vec::new());
/// let state = codetether_paste_buffer::image_recovery::State {
///     composer: codetether_image::paste::State {
///         images: &mut images, input: &mut input, cursor: &mut cursor, status: &mut status,
///     },
///     pastes: &mut pastes,
/// };
/// assert!(state.pastes.is_empty());
/// ```
pub struct State<'a> {
    /// Input, attachments, cursor and status shared with image paste handling.
    pub composer: codetether_image::paste::State<'a>,
    /// Text sidecars containing possible fragments of image data URLs.
    pub pastes: &'a mut Vec<PendingTextPaste>,
}
