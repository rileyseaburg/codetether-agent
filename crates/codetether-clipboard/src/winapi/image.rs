//! Read native Windows PNG, DIBV5, or DIB clipboard pixels as an attachment.
use super::{image_formats, image_read};
use codetether_image::ImageAttachment;

/// Read the clipboard image and return it as a PNG data URL attachment.
/// # Arguments
/// None; reads the current process's system clipboard.
/// # Returns
/// A PNG attachment, or None when unavailable or undecodable.
/// # Examples
/// ```no_run
/// let _image = codetether_clipboard::winapi::get_clipboard_image();
/// ```
pub fn get_clipboard_image() -> Option<ImageAttachment> {
    image_formats::capture(image_read::read)
}
