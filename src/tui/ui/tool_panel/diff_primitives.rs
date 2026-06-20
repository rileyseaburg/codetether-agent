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

pub(super) fn file_field(v: &serde_json::Value) -> String {
    str_field(v, "path", "filePath")
        .or_else(|| v.get("file").and_then(|x| x.as_str()))
        .unwrap_or("unknown")
        .to_string()
}

/// Read a string field accepting either a snake_case or camelCase key.
pub(super) fn str_field<'a>(v: &'a serde_json::Value, snake: &str, camel: &str) -> Option<&'a str> {
    v.get(snake)
        .or_else(|| v.get(camel))
        .and_then(|x| x.as_str())
}
