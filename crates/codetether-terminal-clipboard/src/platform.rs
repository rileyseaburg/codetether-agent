//! Platform text-write operations used before terminal fallback.

#[cfg(not(windows))]
pub(crate) fn platform_set_text(text: &str) -> Result<(), String> {
    arboard::Clipboard::new()
        .and_then(|mut cb| cb.set_text(text.to_string()))
        .map_err(|e| e.to_string())
}

#[cfg(windows)]
pub(crate) fn platform_set_text(text: &str) -> Result<(), String> {
    codetether_clipboard::winapi::set_clipboard_text(text)
        .ok_or_else(|| "clipboard open failed".to_string())
}
