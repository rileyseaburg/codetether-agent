//! Read the clipboard DIB and convert it to a PNG attachment.
use super::{dib::dib_to_bmp, image_encode::encode_png_attachment, image_read::read_cf_dib};
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
    let dib = read_cf_dib()?;
    let bmp = dib_to_bmp(&dib)?;
    encode_png_attachment(&bmp)
}
