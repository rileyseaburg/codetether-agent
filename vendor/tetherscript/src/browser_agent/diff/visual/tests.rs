//! Visual diff unit tests.

use super::compare_rasters;
use crate::browser::RasterImage;

fn img(w: usize, h: usize, rgba: u8) -> RasterImage {
    RasterImage {
        width: w,
        height: h,
        pixels: vec![rgba; w * h * 4],
    }
}

#[test]
fn identical_images_have_no_diff() {
    let a = img(1, 1, 0);
    let diff = compare_rasters(&a, &a);
    assert_eq!(diff.stats.changed_pixels, 0);
    assert_eq!(diff.stats.diff_percentage, 0.0);
    assert!(!diff.size_mismatch);
}

#[test]
fn different_images_report_changed_pixel() {
    let a = img(1, 1, 0);
    let b = img(1, 1, 255);
    let diff = compare_rasters(&a, &b);
    assert_eq!(diff.stats.changed_pixels, 1);
}

#[test]
fn size_mismatch_detected() {
    let a = img(1, 1, 0);
    let b = img(2, 1, 0);
    let diff = compare_rasters(&a, &b);
    assert!(diff.size_mismatch);
}
