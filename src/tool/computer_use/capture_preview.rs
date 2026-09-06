//! Bounded model-facing previews with explicit physical-image coordinate mapping.

use super::capture_limits::{MAX_PREVIEW_BYTES, MAX_PREVIEW_EDGE, pixel_bytes};
use image::{GenericImageView, ImageReader};
use serde_json::{Value, json};
use std::io::Cursor;

pub(crate) struct Preview {
    pub(crate) attachment: Value,
    pub(crate) mapping: Value,
}

pub(crate) fn prepare(png: &[u8], width: u32, height: u32) -> anyhow::Result<Preview> {
    pixel_bytes(i64::from(width), i64::from(height))?;
    anyhow::ensure!(png.starts_with(b"\x89PNG\r\n\x1a\n"), "Capture PNG is missing or invalid");
    if width.max(height) <= MAX_PREVIEW_EDGE && png.len() <= MAX_PREVIEW_BYTES {
        return Ok(result(png, "image/png", width, height, width, height));
    }
    let mut reader = ImageReader::with_format(Cursor::new(png), image::ImageFormat::Png);
    let mut limits = image::Limits::default();
    limits.max_alloc = Some(128 * 1024 * 1024);
    reader.limits(limits);
    let decoded = reader.decode()?;
    anyhow::ensure!(decoded.dimensions() == (width, height), "Capture geometry does not match PNG");
    let preview = decoded.thumbnail(MAX_PREVIEW_EDGE, MAX_PREVIEW_EDGE).to_rgb8();
    let mut bytes = Vec::new();
    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut bytes, 80).encode_image(&preview)?;
    anyhow::ensure!(bytes.len() <= MAX_PREVIEW_BYTES, "Capture preview exceeds transport budget");
    Ok(result(&bytes, "image/jpeg", width, height, preview.width(), preview.height()))
}

fn result(bytes: &[u8], mime: &str, width: u32, height: u32, shown_w: u32, shown_h: u32) -> Preview {
    Preview { attachment: crate::tool::result_images::encoded(bytes, mime), mapping: json!({
        "width": shown_w, "height": shown_h, "mime_type": mime,
        "original_width": width, "original_height": height,
        "image_to_original_scale_x": f64::from(width) / f64::from(shown_w),
        "image_to_original_scale_y": f64::from(height) / f64::from(shown_h),
        "coordinate_note": "Multiply preview coordinates by these scales before using original image/screen coordinates. Capture geometry and OCR boxes remain original-resolution.",
        "encoded_bytes": bytes.len()
    }) }
}

#[cfg(test)]
#[path = "capture_preview_tests.rs"]
mod tests;