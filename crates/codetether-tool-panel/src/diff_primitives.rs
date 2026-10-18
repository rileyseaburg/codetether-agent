//! Shared rendering primitives for tool-panel diff views.
//!
//! Gutter glyphs, headers, truncation markers, single diff lines, and the
//! JSON field accessors used by both the edit-diff and create-preview renderers.

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

pub(super) fn gutter() -> Span<'static> {
    Span::styled("│   ", Style::default().fg(Color::DarkGray).dim())
}

pub(super) fn header(label: &str, color: Color) -> Line<'static> {
    Line::from(vec![
        gutter(),
        Span::styled(format!("📄 {label}"), Style::default().fg(color).bold()),
    ])
}

pub(super) fn marker(n: usize) -> Line<'static> {
    Line::from(vec![
        gutter(),
        Span::styled(
            format!("  ... {n} more lines ..."),
            Style::default().fg(Color::DarkGray).dim(),
        ),
    ])
}

pub(super) fn diff_line(text: &str, prefix: char, color: Color) -> Line<'static> {
    let body = if text.chars().count() > 200 {
        text.chars().take(197).chain("...".chars()).collect()
    } else {
        text.to_string()
    };
    Line::from(vec![
        gutter(),
        Span::styled(format!("{prefix} "), Style::default().fg(color).bold()),
        Span::styled(body, Style::default().fg(color)),
    ])
}

pub(super) use super::diff_fields::{file_field, str_field};
