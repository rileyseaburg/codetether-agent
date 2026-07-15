//! Left-anchored gradient-gutter assistant bubble.
//!
//! Each body line is prefixed by a `│` rail whose color fades along the
//! cyan→magenta gradient as the message grows, so long responses have a
//! visible depth cue. The first line gets a `◉` timeline anchor instead.

use ratatui::{style::Color, text::Line};

use super::timeline::{anchor_line, rail_line};
use crate::tui::ui::gradient::{NEON_CYAN, NEON_MAGENTA, lerp_rgb, rgb_supported};

/// Wrap body rows in a gradient-gutter assistant bubble.
///
/// Preserves the original styled spans (syntax highlighting etc.) and
/// only prepends the gutter rail / anchor glyph.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::bubble_assistant::assistant_bubble_lines;
/// let body = vec![ratatui::text::Line::from("hi")];
/// let lines = assistant_bubble_lines(body);
/// assert_eq!(lines.len(), 1);
/// ```
pub fn assistant_bubble_lines(body: Vec<Line<'static>>) -> Vec<Line<'static>> {
    let total = body.len().max(1);
    body.into_iter()
        .enumerate()
        .map(|(i, line)| {
            let t = i as f32 / total.saturating_sub(1).max(1) as f32;
            let color = gutter_color(t);
            if i == 0 {
                anchor_line(line, color)
            } else {
                rail_line(line, color)
            }
        })
        .collect()
}

fn gutter_color(t: f32) -> Color {
    if rgb_supported() {
        lerp_rgb(NEON_CYAN, NEON_MAGENTA, t)
    } else {
        Color::Cyan
    }
}
