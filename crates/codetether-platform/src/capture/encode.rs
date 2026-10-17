//! BGRA-to-encoded image helper for screen capture.

use anyhow::Result;

/// Convert raw BGRA pixel data to an encoded byte buffer.
///
/// Swaps BGRA → RGBA, then encodes via the `image` crate.
/// # Arguments
/// * `w` - Original pixel width.
/// * `h` - Original pixel height.
/// * `pixels` - Four BGRA bytes for each pixel.
/// * `fmt` - Output image format supported by the configured image encoders.
///
/// # Returns
/// Encoded bytes with the original geometry and an opaque alpha channel.
///
/// # Errors
/// Returns an error for invalid dimensions, mismatched buffer size, or encoding failure.
///
/// # Examples
/// ```
/// use codetether_platform::capture::bgra_to_encoded;
/// let bytes = bgra_to_encoded(1, 1, vec![0, 0, 255, 0], image::ImageFormat::Png).unwrap();
/// assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1a\n");
/// ```
pub fn bgra_to_encoded(
    w: u32,
    h: u32,
    mut pixels: Vec<u8>,
    fmt: image::ImageFormat,
) -> Result<Vec<u8>> {
    let expected = crate::capture::pixel_bytes(i64::from(w), i64::from(h))?;
    anyhow::ensure!(
        pixels.len() == expected,
        "Invalid BGRA capture buffer length"
    );
    for chunk in pixels.chunks_exact_mut(4) {
        chunk.swap(0, 2);
        // GDI's fourth channel is reserved, not usable alpha.
        chunk[3] = 255;
    }
    let img = image::RgbaImage::from_raw(w, h, pixels)
        .ok_or_else(|| anyhow::anyhow!("failed to create image buffer"))?;
    let mut buf = std::io::Cursor::new(Vec::new());
    img.write_to(&mut buf, fmt)?;
    Ok(buf.into_inner())
}

/// Convert raw BGRA pixel data to a PNG byte buffer.
/// # Arguments
/// * `w` - Original pixel width.
/// * `h` - Original pixel height.
/// * `pixels` - Four BGRA bytes for each pixel.
///
/// # Returns
/// PNG bytes with the original dimensions and opaque alpha.
///
/// # Errors
/// Returns an error for invalid geometry, buffer size, or PNG encoding failure.
///
/// # Examples
/// ```
/// use codetether_platform::capture::bgra_to_png;
/// assert!(bgra_to_png(1, 1, vec![]).is_err());
/// ```
pub fn bgra_to_png(w: u32, h: u32, pixels: Vec<u8>) -> Result<Vec<u8>> {
    bgra_to_encoded(w, h, pixels, image::ImageFormat::Png)
}
