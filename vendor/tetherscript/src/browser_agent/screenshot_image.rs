//! Raster crop helpers for page screenshots.

use crate::browser::{RasterImage, Rgba};
use crate::browser_agent::action::BoundingBox;

pub(crate) fn scale_factor(factor: f64) -> usize {
    factor.ceil().max(1.0) as usize
}

pub(crate) fn crop_css(
    source: &RasterImage,
    bounds: BoundingBox,
    scale: usize,
) -> Result<RasterImage, String> {
    let width = extent(bounds.width, scale, "width")?;
    let height = extent(bounds.height, scale, "height")?;
    let mut out = RasterImage::new(width, height, Rgba::WHITE);
    let left = bounds.x.saturating_mul(scale as i64);
    let top = bounds.y.saturating_mul(scale as i64);
    for y in 0..height {
        for x in 0..width {
            copy_pixel(source, &mut out, left + x as i64, top + y as i64, x, y);
        }
    }
    Ok(out)
}

fn copy_pixel(source: &RasterImage, out: &mut RasterImage, x: i64, y: i64, dx: usize, dy: usize) {
    if x < 0 || y < 0 {
        return;
    }
    if let Some(pixel) = source.pixel(x as usize, y as usize) {
        out.set_pixel(dx as i64, dy as i64, pixel);
    }
}

fn extent(value: i64, scale: usize, label: &str) -> Result<usize, String> {
    let value =
        usize::try_from(value.max(1)).map_err(|_| format!("screenshot: negative {label}"))?;
    value
        .checked_mul(scale.max(1))
        .ok_or_else(|| format!("screenshot: {label} overflow"))
}
