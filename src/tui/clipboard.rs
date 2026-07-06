//! Clipboard image and text access for TUI.
//!
//! On Windows, uses the `windows` crate directly (raw-dylib linking) to
//! avoid a linker conflict with `clipboard-win`. On other platforms,
//! delegates to `arboard`.
//!
//! Clipboard access is attempted even under SSH: with X11/Wayland
//! forwarding the remote clipboard IS reachable, and `arboard` fails
//! fast when no display is available, so probing is cheap and safe.

use crate::session::ImageAttachment;

/// Check if we're in an SSH or headless session without clipboard access.
///
/// Used only for help text and guidance messages — actual clipboard
/// reads always probe the real clipboard first.
pub fn is_ssh_or_headless() -> bool {
    super::clipboard_ssh::is_ssh_or_headless()
}

#[cfg(not(windows))]
/// Extract an image from the system clipboard.
///
/// Attempted even over SSH so X11/Wayland-forwarded clipboards work.
/// Returns `None` when no display or no image content is available.
pub fn get_clipboard_image() -> Option<ImageAttachment> {
    crate::image_clipboard::capture_image().ok()
}

#[cfg(windows)]
/// Extract an image from the system clipboard (Windows via raw-dylib).
///
/// Reads `CF_DIB`, wraps it in a BMP header, and re-encodes as PNG.
pub fn get_clipboard_image() -> Option<ImageAttachment> {
    super::clipboard_winapi::get_clipboard_image()
}

#[cfg(not(windows))]
/// Extract plain text from the system clipboard.
///
/// Attempted even over SSH so X11/Wayland-forwarded clipboards work.
pub fn get_clipboard_text() -> Option<String> {
    let mut clipboard = arboard::Clipboard::new().ok()?;
    clipboard.get_text().ok().filter(|t| !t.is_empty())
}

#[cfg(windows)]
/// Extract plain text from the system clipboard (Windows via raw-dylib).
pub fn get_clipboard_text() -> Option<String> {
    super::clipboard_winapi::get_clipboard_text()
}
