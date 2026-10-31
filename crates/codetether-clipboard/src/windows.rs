//! Windows image and text operations using the raw-dylib backend.
use anyhow::{Context, Result};
use codetether_image::ImageAttachment;

/// Read the current system clipboard image (Windows via raw-dylib).
///
/// Reads registered PNG, `CF_DIBV5`, or `CF_DIB` from the native Windows
/// clipboard and normalizes its pixels as a PNG attachment.
///
/// # Errors
///
/// Returns an error when the clipboard is unavailable or contains no image.
#[cfg(windows)]
pub fn capture_image() -> Result<ImageAttachment> {
    crate::winapi::get_clipboard_image().context("clipboard does not contain an image")
}

/// Replace the system clipboard with text (Windows via raw-dylib).
#[cfg(windows)]
pub fn copy_text(text: &str) -> Result<()> {
    crate::winapi::set_clipboard_text(text)
        .context("failed to write text to clipboard via raw-dylib")
}
