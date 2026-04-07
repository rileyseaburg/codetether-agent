//! Clipboard and image paste support for TUI
//!
//! OSC52 escape sequence clipboard and base64 image extraction
//! from terminal clipboard paste events.

use base64::Engine;


fn get_clipboard_image() -> Option<PendingImage> {
    use arboard::Clipboard;
    use image::{ImageBuffer, Rgba};
    use std::io::Cursor;

    let mut clipboard = Clipboard::new().ok()?;
    let img_data = clipboard.get_image().ok()?;

    // arboard gives us RGBA bytes
    let width = img_data.width;
    let height = img_data.height;
    let raw_bytes = img_data.bytes.into_owned();
    let size_bytes = raw_bytes.len();

    // Create an image buffer from the raw RGBA bytes
    let img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width as u32, height as u32, raw_bytes)?;

    // Encode as PNG
    let mut png_bytes: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut png_bytes);
    img_buffer
        .write_to(&mut cursor, image::ImageFormat::Png)
        .ok()?;

    // Base64 encode
    let base64_data =
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &png_bytes);
    let data_url = format!("data:image/png;base64,{}", base64_data);

    Some(PendingImage {
        data_url,
        width,
        height,
        size_bytes,
    })
}


fn osc52_copy(text: &str) -> std::io::Result<()> {
    // OSC52 format: ESC ] 52 ; c ; <base64> BEL
    // Some terminals may disable OSC52 for security; we treat this as best-effort.
    let payload = base64::engine::general_purpose::STANDARD.encode(text.as_bytes());
    let seq = format!("\u{1b}]52;c;{payload}\u{07}");

    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, crossterm::style::Print(seq))?;
    use std::io::Write;
    stdout.flush()?;
    Ok(())
}

fn copy_text_to_clipboard_best_effort(text: &str) -> Result<&'static str, String> {
    if text.trim().is_empty() {
        return Err("empty text".to_string());
    }

    // 1) Try system clipboard first (works locally when a clipboard provider is available)
    match arboard::Clipboard::new().and_then(|mut clipboard| clipboard.set_text(text.to_string())) {
        Ok(()) => return Ok("system clipboard"),
        Err(e) => {
            tracing::debug!(error = %e, "System clipboard unavailable; falling back to OSC52");
        }
    }

    // 2) Fallback: OSC52 (works in many terminals, including remote SSH sessions)
    osc52_copy(text).map_err(|e| format!("osc52 copy failed: {e}"))?;
    Ok("OSC52")
}

