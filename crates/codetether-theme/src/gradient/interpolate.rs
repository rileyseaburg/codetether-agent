//! Linear interpolation between RGB endpoints.
use super::Rgb;
use ratatui::style::Color;

/// Interpolate between two RGB endpoints at `t` in `[0, 1]`.
///
/// # Examples
///
/// ```rust
/// use codetether_theme::gradient::lerp_rgb;
/// use ratatui::style::Color;
/// assert_eq!(lerp_rgb((0, 0, 0), (255, 255, 255), 0.0), Color::Rgb(0, 0, 0));
/// assert_eq!(lerp_rgb((0, 0, 0), (255, 255, 255), 1.0), Color::Rgb(255, 255, 255));
/// ```
pub fn lerp_rgb(from: Rgb, to: Rgb, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    let mix = |a: u8, b: u8| (f32::from(a) + (f32::from(b) - f32::from(a)) * t).round() as u8;
    Color::Rgb(mix(from.0, to.0), mix(from.1, to.1), mix(from.2, to.2))
}
