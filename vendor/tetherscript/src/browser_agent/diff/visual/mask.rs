//! Diff mask generation.

use crate::browser::RasterImage;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffMask {
    pub width: usize,
    pub height: usize,
    pub changed: Vec<bool>,
}

/// Build a boolean mask marking changed pixels between two raster images.
pub fn build_diff_mask(a: &RasterImage, b: &RasterImage) -> DiffMask {
    let width = a.width.max(b.width);
    let height = a.height.max(b.height);
    let mut changed = Vec::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            changed.push(pixel_at(a, x, y) != pixel_at(b, x, y));
        }
    }
    DiffMask {
        width,
        height,
        changed,
    }
}

/// Return the 4-byte RGBA slice for a pixel, or None if out of bounds.
fn pixel_at(img: &RasterImage, x: usize, y: usize) -> Option<[u8; 4]> {
    if x >= img.width || y >= img.height {
        return None;
    }
    let i = (y * img.width + x) * 4;
    if i + 4 > img.pixels.len() {
        return None;
    }
    Some([
        img.pixels[i],
        img.pixels[i + 1],
        img.pixels[i + 2],
        img.pixels[i + 3],
    ])
}
