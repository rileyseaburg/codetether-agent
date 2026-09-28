//! Grouping of status-bar spans into unbreakable key/label units.
//!
//! Keybinding hints are emitted as a key span (`Esc`) followed by a label
//! span (`: Back | `). Packing them separately lets a narrow terminal (for
//! example an iPhone SSH client at ~46 columns) wrap between the two, which
//! renders a dangling `Esc` on one row and `: Back` on the next. Grouping
//! keeps each hint whole so wraps only happen between hints.

use ratatui::text::Span;
use unicode_width::UnicodeWidthStr;

/// Split `spans` into units where a `:`-prefixed label stays with its key.
///
/// # Examples
///
/// ```rust
/// use ratatui::text::Span;
/// use codetether_agent::tui::ui::chat_view::status_glue::units;
///
/// let groups = units(vec![Span::raw("Esc"), Span::raw(": Back | "), Span::raw("x")]);
/// assert_eq!(groups.len(), 2);
/// assert_eq!(groups[0].len(), 2);
/// ```
pub fn units(spans: Vec<Span<'static>>) -> Vec<Vec<Span<'static>>> {
    let mut groups: Vec<Vec<Span<'static>>> = Vec::new();
    for span in spans {
        match groups.last_mut() {
            Some(last) if span.content.starts_with(':') => last.push(span),
            _ => groups.push(vec![span]),
        }
    }
    groups
}

/// Display width of one unit in terminal columns.
///
/// # Examples
///
/// ```rust
/// use ratatui::text::Span;
/// use codetether_agent::tui::ui::chat_view::status_glue::width;
///
/// assert_eq!(width(&[Span::raw("Esc"), Span::raw(": Back")]), 9);
/// ```
pub fn width(unit: &[Span<'static>]) -> usize {
    unit.iter().map(|span| span.content.width()).sum()
}
