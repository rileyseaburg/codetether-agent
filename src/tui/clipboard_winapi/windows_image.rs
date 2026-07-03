//! Read a clipboard image on Windows via the raw-dylib `windows` crate.
//!
//! Fetches `CF_DIB` (a packed device-independent bitmap), wraps it in a BMP
//! file header, then decodes and re-encodes it as a PNG data URL so it can
//! be pasted through an SSH terminal into a remote CodeTether TUI.

use std::io::Cursor;

use base64::Engine;
use windows::Win32::Foundation::HGLOBAL;
use windows::Win32::System::DataExchange::{GetClipboardData, IsClipboardFormatAvailable};
use windows::Win32::System::Memory::{GlobalLock, GlobalSize, GlobalUnlock};

use super::windows::ClipboardGuard;
use super::windows_dib::dib_to_bmp;
use crate::session::ImageAttachment;

const CF_DIB: u32 = 8;

/// Read the clipboard image and return it as a PNG data URL attachment.
pub fn get_clipboard_image() -> Option<ImageAttachment> {
    let dib = read_cf_dib()?;
    let bmp = dib_to_bmp(&dib)?;
    encode_png_attachment(&bmp)
}

/// Copy the raw `CF_DIB` bytes off the clipboard.
fn read_cf_dib() -> Option<Vec<u8>> {
    let _guard = ClipboardGuard::open()?;
    unsafe { IsClipboardFormatAvailable(CF_DIB) }.ok()?;
    let handle = unsafe { GetClipboardData(CF_DIB) }.ok()?;
    let hg = HGLOBAL(handle.0);
    let ptr = unsafe { GlobalLock(hg) };
    if ptr.is_null() {
        return None;
    }
    let len = unsafe { GlobalSize(hg) };
    let bytes = unsafe { std::slice::from_raw_parts(ptr as *const u8, len) }.to_vec();
    unsafe {
        let _ = GlobalUnlock(hg);
    }
    (!bytes.is_empty()).then_some(bytes)
}

/// Decode a BMP buffer and re-encode it as a PNG data URL attachment.
fn encode_png_attachment(bmp: &[u8]) -> Option<ImageAttachment> {
    let decoded = image::load_from_memory_with_format(bmp, image::ImageFormat::Bmp).ok()?;
    let mut png = Vec::new();
    decoded
        .write_to(&mut Cursor::new(&mut png), image::ImageFormat::Png)
        .ok()?;
    let data = base64::engine::general_purpose::STANDARD.encode(&png);
    Some(ImageAttachment {
        data_url: format!("data:image/png;base64,{data}"),
        mime_type: Some("image/png".to_string()),
    })
}
