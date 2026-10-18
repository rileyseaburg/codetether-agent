//! Animated spinner glyphs and neon cycling colors.
//!
//! [`current_spinner_frame`] cycles through a dense braille-arc pattern.
//! [`spinner_color`] returns a neon hue that rotates cyan → magenta → yellow
//! every ~300 ms so the spinner visually "pulses" with color.
//! [`format_elapsed`] renders an [`std::time::Instant`] delta as `MmSS` or `S.s`.

use ratatui::style::Color;

#[path = "spinner_hue.rs"]
pub mod spinner_hue;

const SPINNER: [&str; 8] = ["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"];

/// Neon colors that cycle every ~300 ms.
const NEON: [Color; 6] = [
    Color::Cyan,
    Color::LightCyan,
    Color::Magenta,
    Color::LightMagenta,
    Color::Yellow,
    Color::LightYellow,
];

/// Return the current spinner glyph based on wall-clock time (100 ms steps).
///
/// # Examples
///
/// ```rust
/// use codetether_tui_style::spinner::current_spinner_frame;
/// let frame = current_spinner_frame();
/// assert!(frame.chars().count() >= 1);
/// ```
///
/// # Arguments
/// No arguments.
///
/// # Returns
/// The current static spinner glyph.
pub fn current_spinner_frame() -> &'static str {
    let idx = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        / 100) as usize
        % SPINNER.len();
    SPINNER[idx]
}

/// Return a neon [`Color`] for spinner/accent glow.
///
/// On truecolor terminals this is a smooth continuously-rotating HSV hue
/// (fluid "breathing" glow); otherwise it steps through named neon colors
/// every ~300 ms.
///
/// # Examples
///
/// ```rust
/// use codetether_tui_style::spinner::spinner_color;
/// let _c = spinner_color();
/// ```
///
/// # Arguments
/// No arguments.
///
/// # Returns
/// The current animated or named fallback color.
pub fn spinner_color() -> Color {
    if crate::gradient::rgb_supported() {
        return spinner_hue::hue_color(3_000);
    }
    let idx = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        / 300) as usize
        % NEON.len();
    NEON[idx]
}

pub use crate::elapsed::format_elapsed;
