//! Smooth HSV hue-rotation color for truecolor terminals.
//!
//! Produces a continuously rotating neon hue (full saturation/value)
//! based on wall-clock time, giving spinners and accents a fluid
//! "breathing" glow instead of discrete color steps.

use ratatui::style::Color;

/// A smoothly rotating neon hue with the given rotation `period_ms`.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::spinner::spinner_hue::hue_color;
/// let _c = hue_color(3000);
/// ```
pub fn hue_color(period_ms: u128) -> Color {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let t = (now % period_ms.max(1)) as f32 / period_ms.max(1) as f32;
    hsv_to_rgb(t * 360.0)
}

/// Convert a hue (degrees, s=1, v=1) to an RGB [`Color`].
fn hsv_to_rgb(hue: f32) -> Color {
    let h = (hue % 360.0) / 60.0;
    let x = 1.0 - (h % 2.0 - 1.0).abs();
    let (r, g, b) = match h as u32 {
        0 => (1.0, x, 0.0),
        1 => (x, 1.0, 0.0),
        2 => (0.0, 1.0, x),
        3 => (0.0, x, 1.0),
        4 => (x, 0.0, 1.0),
        _ => (1.0, 0.0, x),
    };
    let scale = |v: f32| (v * 255.0).round() as u8;
    Color::Rgb(scale(r), scale(g), scale(b))
}
