//! Clipboard backend selected by platform.

// Pure byte-parsing logic — compiled everywhere so it is unit-testable on CI.
mod windows_dib;
#[cfg(test)]
mod windows_dib_tests;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
mod windows_image;
#[cfg(windows)]
pub use windows::{get_clipboard_text, set_clipboard_text};
#[cfg(windows)]
pub use windows_image::get_clipboard_image;

#[cfg(not(windows))]
/// Read plain text from the Windows clipboard backend.
pub fn get_clipboard_text() -> Option<String> {
    None
}

#[cfg(not(windows))]
/// Write plain text to the Windows clipboard backend.
pub fn set_clipboard_text(_text: &str) -> Option<()> {
    None
}

#[cfg(not(windows))]
/// Read an image from the Windows clipboard backend.
pub fn get_clipboard_image() -> Option<crate::session::ImageAttachment> {
    None
}
