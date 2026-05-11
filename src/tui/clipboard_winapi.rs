//! Clipboard backend selected by platform.

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::{get_clipboard_text, set_clipboard_text};

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
