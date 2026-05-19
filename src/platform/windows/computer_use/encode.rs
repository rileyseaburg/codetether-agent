//! BGRA-to-encoded image helper for screen capture.

use anyhow::Result;

/// Convert raw BGRA pixel data to an encoded byte buffer.
///
/// Swaps BGRA → RGBA, then encodes via the `image` crate.
pub fn bgra_to_encoded(
    w: u32,
    h: u32,
    mut pixels: Vec<u8>,
    fmt: image::ImageFormat,
) -> Result<Vec<u8>> {
    for chunk in pixels.chunks_exact_mut(4) {
        chunk.swap(0, 2);
    }
    let img = image::RgbaImage::from_raw(w, h, pixels)
        .ok_or_else(|| anyhow::anyhow!("failed to create image buffer"))?;
    let mut buf = std::io::Cursor::new(Vec::new());
    img.write_to(&mut buf, fmt)?;
    Ok(buf.into_inner())
}

/// Convert raw BGRA pixel data to a PNG byte buffer.
pub fn bgra_to_png(w: u32, h: u32, pixels: Vec<u8>) -> Result<Vec<u8>> {
    bgra_to_encoded(w, h, pixels, image::ImageFormat::Png)
}
