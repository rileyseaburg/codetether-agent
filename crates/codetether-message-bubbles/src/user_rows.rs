//! Rounded borders, padded body rows, and right alignment.
use crate::user_style::dim;
use ratatui::{
    style::Color,
    text::{Line, Span},
};
use unicode_width::UnicodeWidthStr;

pub(super) fn border(l: &str, r: &str, inner: usize, color: Color) -> Line<'static> {
    Line::from(Span::styled(
        format!("{l}{}{r}", "─".repeat(inner)),
        dim(color),
    ))
}

pub(super) fn row(text: &str, inner: usize, color: Color) -> Line<'static> {
    let pad = inner.saturating_sub(text.width() + 1);
    Line::from(vec![
        Span::styled("│", dim(color)),
        Span::raw(format!(" {text}{}", " ".repeat(pad))),
        Span::styled("│", dim(color)),
    ])
}

pub(super) fn right_pad(line: Line<'static>, width: usize) -> Line<'static> {
    let used: usize = line.spans.iter().map(|s| s.content.width()).sum();
    let mut spans = vec![Span::raw(" ".repeat(width.saturating_sub(used)))];
    spans.extend(line.spans);
    Line::from(spans)
}
