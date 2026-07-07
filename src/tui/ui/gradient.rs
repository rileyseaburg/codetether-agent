//! Linear RGB gradient helpers for flashy text styling.
//!
//! Provides [`lerp_rgb`] color interpolation, [`gradient_spans`] for
//! per-character gradient text, and [`rgb_supported`] — a cached
//! truecolor-capability probe so render code can fall back gracefully
//! on 8/256-color terminals.

use std::sync::OnceLock;

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;

/// An RGB gradient endpoint.
pub type Rgb = (u8, u8, u8);

/// Neon cyan brand endpoint.
pub const NEON_CYAN: Rgb = (0, 229, 255);
/// Neon magenta brand endpoint.
pub const NEON_MAGENTA: Rgb = (255, 64, 216);

/// Whether the terminal supports 24-bit color (cached after first call).
///
/// # Examples
///
/// ```rust
/// let _ok = codetether_agent::tui::ui::gradient::rgb_supported();
/// ```
pub fn rgb_supported() -> bool {
    static RGB: OnceLock<bool> = OnceLock::new();
    *RGB.get_or_init(|| crate::tui::theme_utils::detect_color_support().supports_rgb())
}

/// Interpolate between two RGB endpoints at `t` in `[0, 1]`.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::gradient::lerp_rgb;
/// use ratatui::style::Color;
/// assert_eq!(lerp_rgb((0, 0, 0), (255, 255, 255), 0.0), Color::Rgb(0, 0, 0));
/// assert_eq!(lerp_rgb((0, 0, 0), (255, 255, 255), 1.0), Color::Rgb(255, 255, 255));
/// ```
pub fn lerp_rgb(from: Rgb, to: Rgb, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    let mix = |a: u8, b: u8| (f32::from(a) + (f32::from(b) - f32::from(a)) * t).round() as u8;
    Color::Rgb(mix(from.0, to.0), mix(from.1, to.1), mix(from.2, to.2))
}

/// Split `text` into per-character spans colored along a gradient.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::gradient::{gradient_spans, NEON_CYAN, NEON_MAGENTA};
/// let spans = gradient_spans("Hi", NEON_CYAN, NEON_MAGENTA, true);
/// assert_eq!(spans.len(), 2);
/// ```
pub fn gradient_spans(text: &str, from: Rgb, to: Rgb, bold: bool) -> Vec<Span<'static>> {
    let chars: Vec<char> = text.chars().collect();
    let last = chars.len().saturating_sub(1).max(1) as f32;
    chars
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let mut style = Style::default().fg(lerp_rgb(from, to, i as f32 / last));
            if bold {
                style = style.add_modifier(Modifier::BOLD);
            }
            Span::styled(c.to_string(), style)
        })
        .collect()
}
