//! Downscale oversized images so vision providers accept them.
//!
//! Anthropic models (Bedrock and direct) reject any image whose longest edge
//! exceeds 2000 px when a request carries many images. Screenshots and
//! generated graphics routinely exceed that, which faults the whole turn.
//! [`fit`] shrinks such images (longest edge [`MAX_EDGE`]) and re-encodes them
//! as JPEG; images that already fit, or cannot be decoded, pass through.

use image::{GenericImageView, ImageReader};
use std::io::Cursor;

mod data_url;
pub use data_url::fit_data_url;

/// Longest edge allowed after fitting, safely under the 2000 px API limit.
pub const MAX_EDGE: u32 = 1920;

/// A fitted image: encoded bytes plus their MIME type.
#[derive(Debug, PartialEq, Eq)]
pub struct Fitted {
    /// Encoded image bytes.
    pub bytes: Vec<u8>,
    /// MIME type of `bytes`, e.g. `image/jpeg`.
    pub mime: String,
}

/// Downscale `bytes` when its longest edge exceeds [`MAX_EDGE`].
///
/// Returns `None` when the image already fits or cannot be decoded, so the
/// caller keeps the original bytes unchanged.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::image_clipboard::fit::{MAX_EDGE, fit};
///
/// let big = image::RgbImage::new(2500, 1000);
/// let mut png = Vec::new();
/// image::DynamicImage::ImageRgb8(big)
///     .write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)
///     .unwrap();
/// let fitted = fit(&png).expect("oversized image is shrunk");
/// assert_eq!(fitted.mime, "image/jpeg");
/// let small = image::load_from_memory(&fitted.bytes).unwrap();
/// assert!(small.width() <= MAX_EDGE && small.height() <= MAX_EDGE);
/// assert!(fit(&fitted.bytes).is_none());
/// ```
pub fn fit(bytes: &[u8]) -> Option<Fitted> {
    let mut reader = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .ok()?;
    let mut limits = image::Limits::default();
    limits.max_alloc = Some(256 * 1024 * 1024);
    reader.limits(limits);
    let decoded = reader.decode().ok()?;
    let (width, height) = decoded.dimensions();
    if width.max(height) <= MAX_EDGE {
        return None;
    }
    let shrunk = decoded.thumbnail(MAX_EDGE, MAX_EDGE).to_rgb8();
    let mut out = Vec::new();
    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, 85)
        .encode_image(&shrunk)
        .ok()?;
    tracing::info!(
        width,
        height,
        fitted_bytes = out.len(),
        "Downscaled oversized image"
    );
    Some(Fitted {
        bytes: out,
        mime: "image/jpeg".into(),
    })
}
