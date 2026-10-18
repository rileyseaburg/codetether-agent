//! Attachment badge suffix for the input area title.
//!
//! [`attachment_suffix`] produces a short string like `| 📷 2 attached`
//! reflecting pending image attachments only. Mid-stream steering is
//! no longer a concept; see the host command handler for `/ask`.

/// Build an attachment-count suffix for the input title.
///
/// # Arguments
/// * `pending_images` - Number of attached images.
///
/// # Returns
/// An empty suffix for zero images or an image-count label.
///
/// # Examples
/// ```
/// assert!(codetether_composer_view::attachment_suffix(0).is_empty());
/// ```
pub fn attachment_suffix(pending_images: usize) -> String {
    if pending_images == 0 {
        return String::new();
    }
    format!(" | 📷 {pending_images} attached ")
}
