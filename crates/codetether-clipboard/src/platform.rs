//! System clipboard capture and text writes through arboard.
use anyhow::{Context, Result};
use codetether_image::ImageAttachment;
use codetether_image::attachment_from_rgba;

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
/// let image = codetether_clipboard::capture_image()?;
/// assert!(image.data_url.starts_with("data:image/png;base64,"));
/// # Ok::<(), anyhow::Error>(())
/// ```
#[cfg(not(windows))]
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
/// codetether_clipboard::copy_text("hello")?;
/// # Ok::<(), anyhow::Error>(())
/// ```
#[cfg(not(windows))]
pub fn copy_text(text: &str) -> Result<()> {
    let mut clipboard = arboard::Clipboard::new().context("system clipboard is not available")?;
    clipboard
        .set_text(text.to_string())
        .context("failed to write image paste text to clipboard")
}
