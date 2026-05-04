//! Diff-style rendering for file edit tool calls.
//!
//! Drives the in-panel "old → new" preview shown for `replace_string_in_file`,
//! `edit_file`, `create_file`, and `write_file`. Kept deliberately small.

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use super::preview_excerpt::preview_excerpt;

const MAX_DIFF_LINES: usize = 30;
const MAX_CREATE_LINES: usize = 15;

/// Render a unified-style diff for `replace_string_in_file` / `edit_file` args.
pub(super) fn push_edit_diff(out: &mut Vec<Line<'static>>, arguments: &str, _w: usize) {
    let Ok(v) = serde_json::from_str::<serde_json::Value>(arguments) else { return };
    let old = v.get("oldString").and_then(|x| x.as_str()).unwrap_or("");
    let new = v.get("newString").and_then(|x| x.as_str()).unwrap_or("");
    out.push(header(&file_field(&v), Color::Yellow));
    push_block(out, old, '-', Color::Red);
    if !old.is_empty() && !new.is_empty() { out.push(Line::from(vec![gutter()])); }
    push_block(out, new, '+', Color::Green);
}

/// Render a preview for `create_file` / `write_file` content.
pub(super) fn push_create_preview(out: &mut Vec<Line<'static>>, arguments: &str, w: usize) {
    let Ok(v) = serde_json::from_str::<serde_json::Value>(arguments) else { return };
    let content = v.get("content").and_then(|x| x.as_str()).unwrap_or("");
    if content.is_empty() { return; }
    let total = content.lines().count();
    out.push(header(&format!("{} ({total} lines)", file_field(&v)), Color::Green));
    let preview = preview_excerpt(content, w);
    for line in preview.lines.iter().take(MAX_CREATE_LINES) { out.push(diff_line(line, '+', Color::Green)); }
    if preview.truncated || preview.lines.len() > MAX_CREATE_LINES {
        out.push(marker(total.saturating_sub(MAX_CREATE_LINES)));
    }
}

fn push_block(out: &mut Vec<Line<'static>>, text: &str, prefix: char, color: Color) {
    let total = text.lines().count();
    for line in text.lines().take(MAX_DIFF_LINES) { out.push(diff_line(line, prefix, color)); }
    if total > MAX_DIFF_LINES { out.push(marker(total - MAX_DIFF_LINES)); }
}

fn file_field(v: &serde_json::Value) -> String {
    v.get("filePath").or_else(|| v.get("path")).and_then(|x| x.as_str()).unwrap_or("unknown").to_string()
}

fn gutter() -> Span<'static> { Span::styled("│   ", Style::default().fg(Color::DarkGray).dim()) }

fn header(label: &str, color: Color) -> Line<'static> {
    Line::from(vec![gutter(), Span::styled(format!("📄 {label}"), Style::default().fg(color).bold())])
}

fn marker(n: usize) -> Line<'static> {
    Line::from(vec![gutter(), Span::styled(format!("  ... {n} more lines ..."), Style::default().fg(Color::DarkGray).dim())])
}

fn diff_line(text: &str, prefix: char, color: Color) -> Line<'static> {
    let body = if text.chars().count() > 200 { text.chars().take(197).chain("...".chars()).collect() } else { text.to_string() };
    Line::from(vec![gutter(), Span::styled(format!("{prefix} "), Style::default().fg(color).bold()), Span::styled(body, Style::default().fg(color))])
}
