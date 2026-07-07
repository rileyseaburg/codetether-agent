//! Truecolor heat-gradient fill for the context gauge.

use ratatui::{style::Style, text::Span};

use crate::tui::ui::gradient::lerp_rgb;

const GREEN: (u8, u8, u8) = (40, 200, 80);
const RED: (u8, u8, u8) = (230, 40, 40);

/// Build per-cell heat-gradient spans for the context gauge bar.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::context_gauge::context_gauge_heat::heat_gradient_spans;
/// let spans = heat_gradient_spans(4, 8, 50);
/// assert_eq!(spans.len(), 8 + 2);
/// ```
pub fn heat_gradient_spans(filled: usize, cells: usize, pct: usize) -> Vec<Span<'static>> {
    let denom = cells.saturating_sub(1).max(1) as f32;
    let mut spans = vec![Span::raw("▕")];
    for i in 0..cells {
        let color = lerp_rgb(GREEN, RED, i as f32 / denom);
        let ch = if i < filled { "█" } else { "░" };
        spans.push(Span::styled(ch, Style::default().fg(color)));
    }
    spans.push(Span::raw(format!("▏ {pct}%")));
    spans
}
