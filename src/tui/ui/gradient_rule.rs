//! Chunked gradient horizontal rules (separator lines).
//!
//! Renders a repeated glyph across `width` columns as ~12 gradient
//! segments instead of per-character spans, keeping span counts low
//! for long separator lines that appear between every chat entry.

use ratatui::style::Style;
use ratatui::text::{Line, Span};

use super::gradient::{Rgb, lerp_rgb};

const SEGMENTS: usize = 12;

/// Build a horizontal rule of `pattern` glyphs fading `from` → `to`.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::gradient_rule::gradient_rule;
/// let line = gradient_rule("─", 24, (0, 229, 255), (60, 20, 70));
/// assert!(!line.spans.is_empty());
/// ```
pub fn gradient_rule(pattern: &str, width: usize, from: Rgb, to: Rgb) -> Line<'static> {
    let width = width.max(1);
    let segs = SEGMENTS.min(width);
    let base = width / segs;
    let extra = width % segs;
    let denom = segs.saturating_sub(1).max(1) as f32;
    let mut spans = Vec::with_capacity(segs);
    for i in 0..segs {
        let len = base + usize::from(i < extra);
        if len == 0 {
            continue;
        }
        let color = lerp_rgb(from, to, i as f32 / denom);
        spans.push(Span::styled(
            pattern.repeat(len),
            Style::default().fg(color),
        ));
    }
    Line::from(spans)
}
