//! Per-character gradient styling.
use super::{Rgb, lerp_rgb};
use ratatui::{
    style::{Modifier, Style},
    text::Span,
};

/// Split `text` into per-character spans colored along a gradient.
///
/// # Examples
///
/// ```rust
/// use codetether_theme::gradient::{gradient_spans, NEON_CYAN, NEON_MAGENTA};
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
