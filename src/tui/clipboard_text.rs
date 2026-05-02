//! Text clipboard helpers: system clipboard + OSC52 escape sequence.

/// Copy text to the system clipboard, with OSC52 as a secondary option.
/// Returns the method name on success.
pub fn copy_text(text: &str) -> Result<&'static str, String> {
    if text.trim().is_empty() {
        return Err("empty text".to_string());
    }
    match platform_set_text(text) {
        Ok(()) => return Ok("system clipboard"),
        Err(e) => {
            tracing::debug!(error = %e, "clipboard write failed, trying OSC52");
        }
    }
    osc52_copy(text).map_err(|e| format!("osc52: {e}"))?;
    Ok("OSC52")
}

#[cfg(not(windows))]
fn platform_set_text(text: &str) -> Result<(), String> {
    arboard::Clipboard::new()
        .and_then(|mut cb| cb.set_text(text.to_string()))
        .map_err(|e| e.to_string())
}

#[cfg(windows)]
fn platform_set_text(text: &str) -> Result<(), String> {
    crate::tui::clipboard_winapi::set_clipboard_text(text)
        .ok_or_else(|| "clipboard open failed".to_string())
}

fn osc52_copy(text: &str) -> std::io::Result<()> {
    use base64::Engine;
    use std::io::Write;
    let b64 = base64::engine::general_purpose::STANDARD.encode(text.as_bytes());
    let seq = format!("\u{1b}]52;c;{b64}\u{07}");
    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, crossterm::style::Print(seq))?;
    stdout.flush()
}
