//! Process-wide cached truecolor detection.
use std::sync::OnceLock;

/// Whether the terminal supports 24-bit color (cached after first call).
///
/// # Examples
///
/// ```rust
/// let _ok = codetether_theme::gradient::rgb_supported();
/// ```
pub fn rgb_supported() -> bool {
    static RGB: OnceLock<bool> = OnceLock::new();
    *RGB.get_or_init(|| crate::terminal::detect_color_support().supports_rgb())
}
