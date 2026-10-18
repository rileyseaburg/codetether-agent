//! Classifier-driven edit-diff rendering.
//!
//! Renders `replace_string_in_file` / `edit_file` arguments as a colored diff:
//! pure deletions red (`-`), pure additions green (`+`), and paired
//! modifications yellow (`~`), using [`super::line_diff`] for classification.

use ratatui::{style::Color, text::Line};

use super::diff_primitives::{diff_line, file_field, header, marker, str_field};
use super::line_diff::{self, LineKind};

const MAX_DIFF_LINES: usize = 30;

/// Render a unified-style diff for `replace_string_in_file` / `edit_file` args.
pub(super) fn push_edit_diff(out: &mut Vec<Line<'static>>, arguments: &str, _w: usize) {
    let Ok(v) = serde_json::from_str::<serde_json::Value>(arguments) else {
        return;
    };
    let old = str_field(&v, "old_string", "oldString").unwrap_or("");
    let new = str_field(&v, "new_string", "newString").unwrap_or("");
    out.push(header(&file_field(&v), Color::Yellow));
    let lines = line_diff::classify(old, new);
    for line in lines.iter().take(MAX_DIFF_LINES) {
        let (prefix, color) = style_for(line.kind);
        out.push(diff_line(&line.text, prefix, color));
    }
    if lines.len() > MAX_DIFF_LINES {
        out.push(marker(lines.len() - MAX_DIFF_LINES));
    }
}

/// Map a classified line kind to its prefix glyph and color.
fn style_for(kind: LineKind) -> (char, Color) {
    match kind {
        LineKind::Delete => ('-', Color::Red),
        LineKind::Insert => ('+', Color::Green),
        LineKind::ChangeOld | LineKind::ChangeNew => ('~', Color::Yellow),
    }
}
