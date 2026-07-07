//! Position-aware gradient scrollbar thumb.
//!
//! The thumb color interpolates from cyan (top) to magenta (bottom)
//! as `scroll_offset` approaches `max_offset`, giving a spatial sense
//! of where you are in the conversation without any numbers.

use ratatui::{style::Style, text::Span};

use crate::tui::ui::gradient::{NEON_CYAN, NEON_MAGENTA, lerp_rgb, rgb_supported};

/// Build a vertical scrollbar column with a position-aware gradient thumb.
///
/// Returns empty when content fits in the viewport.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::scroll_indicator::scrollbar_column;
/// let col = scrollbar_column(100, 10, 0);
/// assert_eq!(col.len(), 10);
/// assert!(scrollbar_column(5, 10, 0).is_empty());
/// ```
pub fn scrollbar_column(
    total_lines: usize,
    viewport_height: usize,
    scroll_offset: usize,
) -> Vec<Span<'static>> {
    if total_lines <= viewport_height || viewport_height == 0 {
        return Vec::new();
    }
    let max_offset = total_lines - viewport_height;
    let offset = scroll_offset.min(max_offset);
    let thumb = (offset * viewport_height.saturating_sub(1)) / max_offset.max(1);
    let t = offset as f32 / max_offset.max(1) as f32;
    (0..viewport_height)
        .map(|row| {
            if row == thumb {
                thumb_span(t)
            } else {
                track_span()
            }
        })
        .collect()
}

fn thumb_span(t: f32) -> Span<'static> {
    let color = if rgb_supported() {
        lerp_rgb(NEON_CYAN, NEON_MAGENTA, t)
    } else {
        ratatui::style::Color::Cyan
    };
    Span::styled("█", Style::default().fg(color))
}

fn track_span() -> Span<'static> {
    use ratatui::style::Color;
    Span::styled("│", Style::default().fg(Color::DarkGray))
}
