//! Existing terminal capability and fallback checks.

use super::fallback::fallback_color;
use super::*;
use crate::ColorDef;

#[test]
fn test_color_support_detection() {
    let support = detect_color_support();
    // Just ensure it doesn't panic
    assert!(matches!(
        support,
        ColorSupport::Monochrome
            | ColorSupport::Ansi8
            | ColorSupport::Ansi256
            | ColorSupport::TrueColor
    ));
}

#[test]
fn test_fallback_color() {
    let support = ColorSupport::Ansi8;
    let rgb_color = ColorDef::Rgb(255, 0, 0);
    let fallback = fallback_color(&rgb_color, &support);
    assert!(matches!(fallback, ColorDef::Named(_)));
}
