//! Text clipboard helpers: system clipboard + OSC52 fallback.

/// Copy text to clipboard via `arboard`, falling back to OSC52.
/// Returns the method name on success.
pub fn copy_text(text: &str) -> Result<&'static str, String> {
    if text.trim().is_empty() {
        return Err("empty text".to_string());
    }
    match arboard::Clipboard::new()
        .and_then(|mut cb| cb.set_text(text.to_string()))
    {
        Ok(()) => return Ok("system clipboard"),
        Err(e) => {
            tracing::debug!(error = %e, "arboard unavailable, trying OSC52");
        }
    }
    osc52_copy(text).map_err(|e| format!("osc52: {e}"))?;
    Ok("OSC52")
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
