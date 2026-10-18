//! Infer terminal color capabilities from standard environment hints.
use super::ColorSupport;

/// Check terminal color support via environment detection
/// # Arguments
/// None; inspects COLORTERM and TERM without changing the environment.
/// # Returns
/// Detected capabilities, defaulting to basic ANSI colors.
/// # Examples
/// ```
/// let _support = codetether_theme::terminal::detect_color_support();
/// ```
pub fn detect_color_support() -> ColorSupport {
    // Check COLORTERM for truecolor
    if let Ok(colorterm) = std::env::var("COLORTERM")
        && (colorterm == "truecolor" || colorterm == "24bit")
    {
        return ColorSupport::TrueColor;
    }

    // Check TERM for 256 color support
    if let Ok(term) = std::env::var("TERM") {
        if term.contains("256color") || term.contains("256") {
            return ColorSupport::Ansi256;
        }
        if term.contains("color") || term.starts_with("xterm") || term.starts_with("screen") {
            return ColorSupport::Ansi8;
        }
    }

    // Default to 8 colors (most common minimum)
    ColorSupport::Ansi8
}
