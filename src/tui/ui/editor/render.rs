//! Renders editor content to ratatui lines with a line-number gutter.
//!
//! [`editor_lines`] is a pure transform from a backend's visible lines into
//! styled [`Line`]s, so it can be unit-tested without a live terminal. Drawing
//! to a [`Frame`](ratatui::Frame) is a thin wrapper layered on top elsewhere.

use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

use super::backend::EditorBackend;

/// Builds gutter+text lines for `height` rows starting at logical line `top`,
/// with each line's text scrolled left by `hscroll` columns.
pub fn editor_lines<B: EditorBackend>(
    backend: &B,
    top: usize,
    height: usize,
    hscroll: usize,
) -> Vec<Line<'static>> {
    let width = gutter_width(backend.line_count());
    backend
        .visible_lines(top, height)
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            let num = top + i + 1;
            let gutter = Span::styled(
                format!("{num:>width$} "),
                Style::default().fg(Color::DarkGray),
            );
            let text: String = line.cells.iter().skip(hscroll).map(|c| c.ch).collect();
            Line::from(vec![gutter, Span::raw(text)])
        })
        .collect()
}

/// Number of digits needed to show the largest line number.
pub(super) fn gutter_width(line_count: usize) -> usize {
    line_count.max(1).to_string().len()
}

/// Usable text columns inside `area_width` after borders and the gutter.
pub fn text_width<B: EditorBackend>(backend: &B, area_width: u16) -> usize {
    let gutter = gutter_width(backend.line_count()) + 1;
    (area_width as usize).saturating_sub(2 + gutter)
}
