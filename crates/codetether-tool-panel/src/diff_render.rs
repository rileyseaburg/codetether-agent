//! Diff-style rendering for file edit tool calls.
//!
//! Drives the in-panel "old → new" preview shown for `replace_string_in_file`,
//! `edit_file`, `create_file`, and `write_file`. Edit diffs live in
//! [`super::edit_diff`]; this module owns the create/write preview.

use ratatui::{style::Color, text::Line};

use super::diff_primitives::{diff_line, file_field, header, marker};
use super::preview_excerpt::preview_excerpt;

pub(super) use super::edit_diff::push_edit_diff;

const MAX_CREATE_LINES: usize = 15;

/// Render a preview for `create_file` / `write_file` content.
pub(super) fn push_create_preview(out: &mut Vec<Line<'static>>, arguments: &str, w: usize) {
    let Ok(v) = serde_json::from_str::<serde_json::Value>(arguments) else {
        return;
    };
    let content = v.get("content").and_then(|x| x.as_str()).unwrap_or("");
    if content.is_empty() {
        return;
    }
    let total = content.lines().count();
    out.push(header(
        &format!("{} ({total} lines)", file_field(&v)),
        Color::Green,
    ));
    let preview = preview_excerpt(content, w);
    for line in preview.lines.iter().take(MAX_CREATE_LINES) {
        out.push(diff_line(line, '+', Color::Green));
    }
    if preview.truncated || preview.lines.len() > MAX_CREATE_LINES {
        out.push(marker(total.saturating_sub(MAX_CREATE_LINES)));
    }
}
