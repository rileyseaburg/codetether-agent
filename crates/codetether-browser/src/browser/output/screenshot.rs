//! PNG screenshot payloads produced from native PPM captures.
use image::{ImageFormat, ImageReader};
use std::io::Cursor;

/// An encoded PNG screenshot.
///
/// # Examples
/// ```
/// use codetether_browser::output::ScreenshotData;
/// let shot = ScreenshotData::from_ppm(b"P6\n1 1\n255\n\xff\x00\x00").unwrap();
/// assert!(shot.bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
/// ```
#[derive(Debug, Clone)]
pub struct ScreenshotData {
    /// PNG-encoded bytes matching the tool's `image/png` MIME label.
    pub bytes: Vec<u8>,
}

impl ScreenshotData {
    /// Encode a native renderer PPM capture as PNG.
    ///
    /// # Arguments
    /// * `bytes` - PPM bytes from a page or element capture.
    /// # Returns
    /// A PNG screenshot ready for file writing and provider attachment.
    /// # Errors
    /// Returns an error for invalid PPM, decoding limits, or PNG encoding failure.
    /// # Examples
    /// ```
    /// assert!(codetether_browser::output::ScreenshotData::from_ppm(b"invalid").is_err());
    /// ```
    pub fn from_ppm(bytes: &[u8]) -> Result<Self, image::ImageError> {
        let decoded = ImageReader::with_format(Cursor::new(bytes), ImageFormat::Pnm).decode()?;
        let mut bytes = Vec::new();
        decoded.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)?;
        Ok(Self { bytes })
    }
}

#[cfg(test)]
mod tests;
