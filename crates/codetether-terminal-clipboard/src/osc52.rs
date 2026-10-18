//! Encode and flush terminal OSC52 clipboard output.

pub(crate) fn osc52_copy(text: &str) -> std::io::Result<()> {
    use base64::Engine;
    use std::io::Write;
    let b64 = base64::engine::general_purpose::STANDARD.encode(text.as_bytes());
    let seq = format!("\u{1b}]52;c;{b64}\u{07}");
    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, crossterm::style::Print(seq))?;
    stdout.flush()
}
