//! Width-aware packing of status-bar spans into wrapped lines.
//!
//! The bottom status bar contains many badges. On narrow terminals a
//! single flat [`Line`] runs off the right edge (clipping). This module
//! greedily packs spans into as many [`Line`]s as needed to fit `width`,
//! so every badge stays visible without manual zoom-out.

use ratatui::text::{Line, Span};
use unicode_width::UnicodeWidthStr;

/// Greedily pack `spans` into lines no wider than `width` columns.
///
/// A span wider than `width` on its own is placed on its own line
/// (it will still clip, but cannot be split safely).
///
/// # Examples
///
/// ```rust
/// use ratatui::text::Span;
/// use codetether_agent::tui::ui::chat_view::status_pack::pack_spans;
///
/// let spans = vec![Span::raw("aaaa"), Span::raw("bbbb"), Span::raw("cccc")];
/// let lines = pack_spans(spans, 8);
/// assert!(lines.len() >= 2);
/// ```
pub fn pack_spans(spans: Vec<Span<'static>>, width: u16) -> Vec<Line<'static>> {
    let width = width.max(1) as usize;
    let mut lines: Vec<Line<'static>> = Vec::new();
    let mut current: Vec<Span<'static>> = Vec::new();
    let mut used = 0usize;

    for span in spans {
        let w = span.content.width();
        if used + w > width && !current.is_empty() {
            lines.push(Line::from(std::mem::take(&mut current)));
            used = 0;
        }
        used += w;
        current.push(span);
    }
    if !current.is_empty() {
        lines.push(Line::from(current));
    }
    if lines.is_empty() {
        lines.push(Line::from(""));
    }
    lines
}

/// Pack three status groups (hints, badges, metrics) into width-bounded lines.
///
/// # Examples
///
/// ```rust
/// use ratatui::text::Span;
/// use codetether_agent::tui::ui::chat_view::status_pack::pack_stacked;
///
/// let lines = pack_stacked(
///     vec![Span::raw("hints")],
///     vec![Span::raw("badge")],
///     vec![Span::raw("metric")],
///     80,
/// );
/// assert_eq!(lines.len(), 3);
/// ```
pub fn pack_stacked(
    hints: Vec<Span<'static>>,
    badges: Vec<Span<'static>>,
    metrics: Vec<Span<'static>>,
    width: u16,
) -> Vec<Line<'static>> {
    let mut lines = pack_spans(hints, width);
    lines.extend(pack_spans(badges, width));
    lines.extend(pack_spans(metrics, width));
    lines
}
