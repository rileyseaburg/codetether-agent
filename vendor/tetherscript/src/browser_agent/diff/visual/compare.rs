//! Pixel-by-pixel raster comparison.

use super::mask::{build_diff_mask, DiffMask};
use super::stats::DiffStats;
use crate::browser::RasterImage;

#[derive(Debug, Clone, PartialEq)]
pub struct DiffResult {
    pub stats: DiffStats,
    pub mask: DiffMask,
    pub size_mismatch: bool,
}

/// Compare two raster images pixel-by-pixel.
pub fn compare_rasters(a: &RasterImage, b: &RasterImage) -> DiffResult {
    let mask = build_diff_mask(a, b);
    let stats = stats_for(&mask);
    let size_mismatch = a.width != b.width || a.height != b.height;
    DiffResult {
        stats,
        mask,
        size_mismatch,
    }
}

fn stats_for(mask: &DiffMask) -> DiffStats {
    let changed = mask.changed.iter().filter(|&&c| c).count();
    let total = mask.width * mask.height;
    let pct = if total == 0 {
        0.0
    } else {
        changed as f64 * 100.0 / total as f64
    };
    DiffStats {
        total_pixels: total,
        changed_pixels: changed,
        max_color_distance: 1020,
        diff_percentage: pct,
    }
}
