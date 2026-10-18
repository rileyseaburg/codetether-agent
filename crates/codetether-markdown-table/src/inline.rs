//! Inline backtick code rendering within a single table cell.

use ratatui::{
    style::{Color, Style},
    text::Span,
};

/// Split a padded cell into spans, styling `` `code` `` segments dim.
///
/// The cell text is already width-padded; we tokenize on backticks so the
/// padding is preserved while code runs get a `DarkGray` tint.
pub(crate) fn cell_spans(text: &str, base: Style) -> Vec<Span<'static>> {
    if !text.contains('`') {
        return vec![Span::styled(text.to_string(), base)];
    }
    let code = Style::default().fg(Color::DarkGray);
    let mut spans = Vec::new();
    let mut in_code = false;
    for (i, seg) in text.split('`').enumerate() {
        if i > 0 {
            in_code = !in_code;
        }
        if seg.is_empty() {
            continue;
        }
        let style = if in_code { code } else { base };
        spans.push(Span::styled(seg.to_string(), style));
    }
    spans
}
