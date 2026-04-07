//! Clipboard image paste support for TUI.
//!
//! Reads image data from the system clipboard and converts it to an
//! `ImageAttachment` suitable for sending with a chat message.

use base64::Engine;
use std::io::Cursor;

use crate::session::ImageAttachment;

/// Check if we're in an SSH or headless session without clipboard access.
fn is_ssh_or_headless() -> bool {
    std::env::var("SSH_CONNECTION").is_ok()
        || std::env::var("SSH_TTY").is_ok()
        || (std::env::var("TERM").ok().map_or(false, |t| t.starts_with("xterm"))
            && std::env::var("DISPLAY").is_err()
            && std::env::var("WAYLAND_DISPLAY").is_err())
}

/// Extract an image from the system clipboard, returning `None` when
/// unavailable (SSH/headless, no clipboard, or no image content).
pub fn get_clipboard_image() -> Option<ImageAttachment> {
    if is_ssh_or_headless() {
        return None;
    }

    let mut clipboard = arboard::Clipboard::new().ok()?;
    let img_data = clipboard.get_image().ok()?;

    let width = img_data.width;
    let height = img_data.height;
    let raw_bytes = img_data.bytes.into_owned();

    let buf: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        image::ImageBuffer::from_raw(width as u32, height as u32, raw_bytes)?;

    let mut png_bytes = Vec::new();
    buf.write_to(&mut Cursor::new(&mut png_bytes), image::ImageFormat::Png)
        .ok()?;

    let b64 = base64::engine::general_purpose::STANDARD.encode(&png_bytes);
    Some(ImageAttachment {
        data_url: format!("data:image/png;base64,{b64}"),
        mime_type: Some("image/png".to_string()),
    })
}
