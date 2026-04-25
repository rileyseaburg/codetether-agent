//! Clipboard image paste support for TUI.
//!
//! Reads image data from the system clipboard and converts it to an
//! `ImageAttachment` suitable for sending with a chat message.

use crate::session::ImageAttachment;

/// Check if we're in an SSH or headless session without clipboard access.
pub fn is_ssh_or_headless() -> bool {
    super::clipboard_ssh::is_ssh_or_headless()
}

/// Extract an image from the system clipboard, returning `None` when
/// unavailable (SSH/headless, no clipboard, or no image content).
pub fn get_clipboard_image() -> Option<ImageAttachment> {
    if is_ssh_or_headless() {
        return None;
    }
    crate::image_clipboard::capture_image().ok()
}

/// Extract plain text from the system clipboard, returning `None` when
/// unavailable (SSH/headless, no clipboard, or no text content).
pub fn get_clipboard_text() -> Option<String> {
    if is_ssh_or_headless() {
        return None;
    }
    let mut clipboard = arboard::Clipboard::new().ok()?;
    clipboard.get_text().ok().filter(|t| !t.is_empty())
}
