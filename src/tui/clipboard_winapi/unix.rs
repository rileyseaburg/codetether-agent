//! Non-Windows clipboard compatibility for the WinAPI module path.

/// Read plain text from the platform clipboard.
pub fn get_clipboard_text() -> Option<String> {
    None
}

/// Write plain text to the platform clipboard.
pub fn set_clipboard_text(_text: &str) -> Option<()> {
    None
}
