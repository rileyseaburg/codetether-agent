//! Capture and write system clipboard data for the image bridge.

use std::io::Cursor;

use anyhow::{Context, Result};
use base64::Engine;

use crate::session::ImageAttachment;

/// Read the current system clipboard image as a PNG data URL.
///
/// # Errors
///
/// Returns an error when the clipboard is unavailable, does not contain an
/// image, or the image cannot be encoded as PNG.
///
/// # Examples
///
/// ```rust,no_run
/// let image = codetether_agent::image_clipboard::capture_image()?;
/// assert!(image.data_url.starts_with("data:image/png;base64,"));
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn capture_image() -> Result<ImageAttachment> {
    let mut clipboard = arboard::Clipboard::new().context("system clipboard is not available")?;
    let image = clipboard
        .get_image()
        .context("clipboard does not contain an image")?;
    attachment_from_rgba(image.width, image.height, image.bytes.into_owned())
        .context("clipboard image could not be encoded as PNG")
}

/// Replace the system clipboard with text.
///
/// # Errors
///
/// Returns an error when the clipboard cannot be opened or written.
///
/// # Examples
///
/// ```rust,no_run
/// codetether_agent::image_clipboard::copy_text("hello")?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn copy_text(text: &str) -> Result<()> {
    let mut clipboard = arboard::Clipboard::new().context("system clipboard is not available")?;
    clipboard
        .set_text(text.to_string())
        .context("failed to write image paste text to clipboard")
}

fn attachment_from_rgba(width: usize, height: usize, bytes: Vec<u8>) -> Option<ImageAttachment> {
    let buf: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        image::ImageBuffer::from_raw(width as u32, height as u32, bytes)?;
    let mut png = Vec::new();
    buf.write_to(&mut Cursor::new(&mut png), image::ImageFormat::Png)
        .ok()?;
    let data = base64::engine::general_purpose::STANDARD.encode(&png);
    Some(ImageAttachment {
        data_url: format!("data:image/png;base64,{data}"),
        mime_type: Some("image/png".to_string()),
    })
}
