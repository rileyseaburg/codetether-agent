use crate::ImageAttachment;
/// Borrowed composer fields changed by image paste ingestion.
/// # Examples
/// ```
/// let (mut images, mut input, mut cursor, mut status) = (Vec::new(), String::new(), 0, String::new());
/// let state = codetether_image::paste::State {
///     images: &mut images, input: &mut input, cursor: &mut cursor, status: &mut status,
/// };
/// assert!(state.images.is_empty());
/// ```
pub struct State<'a> {
    /// Pending attachments in composer order.
    pub images: &'a mut Vec<ImageAttachment>,
    /// Remaining composer text after extracting embedded images.
    pub input: &'a mut String,
    /// Composer cursor measured in characters.
    pub cursor: &'a mut usize,
    /// Status shown after attachment or rejection.
    pub status: &'a mut String,
}
