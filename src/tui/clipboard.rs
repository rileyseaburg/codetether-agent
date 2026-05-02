//! Clipboard image and text access for TUI.
//!
//! On Windows, uses the `windows` crate directly (raw-dylib linking) to
//! avoid a linker conflict with `clipboard-win`. On other platforms,
//! delegates to `arboard`.

use crate::session::ImageAttachment;

/// Check if we're in an SSH or headless session without clipboard access.
pub fn is_ssh_or_headless() -> bool {
    super::clipboard_ssh::is_ssh_or_headless()
}

#[cfg(not(windows))]
/// Extract an image from the system clipboard.
pub fn get_clipboard_image() -> Option<ImageAttachment> {
    if is_ssh_or_headless() { return None; }
    crate::image_clipboard::capture_image().ok()
}

#[cfg(windows)]
/// Extract an image from the system clipboard (Windows).
///
/// Image clipboard requires BITMAPV5HEADER parsing which is not yet
/// implemented for the Windows raw-dylib path. Returns `None` so the
/// TUI falls through to the "clipboard unavailable" status message.
pub fn get_clipboard_image() -> Option<ImageAttachment> {
    if is_ssh_or_headless() { return None; }
    None
}

#[cfg(not(windows))]
/// Extract plain text from the system clipboard.
pub fn get_clipboard_text() -> Option<String> {
    if is_ssh_or_headless() { return None; }
    let mut clipboard = arboard::Clipboard::new().ok()?;
    clipboard.get_text().ok().filter(|t| !t.is_empty())
}

#[cfg(windows)]
/// Extract plain text from the system clipboard (Windows via raw-dylib).
pub fn get_clipboard_text() -> Option<String> {
    if is_ssh_or_headless() { return None; }
    super::clipboard_winapi::get_clipboard_text()
}
