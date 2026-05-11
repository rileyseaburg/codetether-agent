//! Clipboard backend selected by platform-specific submodules.

#[cfg(windows)]
#[path = "clipboard_winapi/windows.rs"]
mod imp;
#[cfg(not(windows))]
#[path = "clipboard_winapi/unix.rs"]
mod imp;

pub use imp::{get_clipboard_text, set_clipboard_text};
