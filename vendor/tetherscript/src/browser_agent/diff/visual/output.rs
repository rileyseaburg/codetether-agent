//! Diff image output with changed pixels highlighted in red.

use super::mask::DiffMask;
use crate::browser::RasterImage;

/// Produce a diff image where changed pixels are highlighted red.
pub fn diff_image(base: &RasterImage, mask: &DiffMask) -> RasterImage {
    let mut pixels = base.pixels.clone();
    for y in 0..base.height.min(mask.height) {
        for x in 0..base.width.min(mask.width) {
            let mi = y * mask.width + x;
            if mask.changed[mi] {
                let pi = (y * base.width + x) * 4;
                if pi + 4 <= pixels.len() {
                    pixels[pi] = 255; // R
                    pixels[pi + 1] = 0; // G
                    pixels[pi + 2] = 0; // B
                    pixels[pi + 3] = 255; // A
                }
            }
        }
    }
    RasterImage {
        width: base.width,
        height: base.height,
        pixels,
    }
}
