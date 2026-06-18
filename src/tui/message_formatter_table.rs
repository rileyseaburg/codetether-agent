//! GitHub-flavored markdown table detection and rendering.
//!
//! Detects pipe-delimited tables (a row, a `---|---` separator, then body
//! rows) and renders them as a bordered, column-aligned box. Falls back to
//! plain text when the buffered lines are not a valid table.

use ratatui::text::Line;

use super::MessageFormatter;

#[path = "message_formatter_table_parse.rs"]
mod parse;
#[path = "message_formatter_table_render.rs"]
mod render;
#[path = "message_formatter_table_util.rs"]
mod util;

#[cfg(test)]
#[path = "message_formatter_table_feature_tests.rs"]
mod feature_tests;
#[cfg(test)]
#[path = "message_formatter_table_tests.rs"]
mod tests;

pub(crate) use parse::{is_separator_row, is_table_row};

/// Render buffered candidate lines as a table when valid.
///
/// Returns `Some(lines)` when `buf` forms a valid GFM table (header row,
/// separator, optional body), otherwise `None` so the caller can fall back
/// to plain rendering.
pub(super) fn render_table(buf: &[String], max_width: usize) -> Option<Vec<Line<'static>>> {
    if buf.len() < 2 || !is_separator_row(&buf[1]) {
        return None;
    }
    render::table(buf, max_width)
}

/// Flush buffered table rows into `out`.
///
/// Renders a valid GFM table as a bordered box, otherwise falls back to
/// plain per-line formatting. Clears `buf` when done.
pub(super) fn flush(
    fmt: &MessageFormatter,
    buf: &mut Vec<String>,
    out: &mut Vec<Line<'static>>,
    role: &str,
) {
    if buf.is_empty() {
        return;
    }
    let width = fmt.max_width().saturating_sub(4);
    if let Some(rendered) = render_table(buf, width) {
        out.extend(rendered);
    } else {
        for raw in buf.iter() {
            let formatted = super::message_formatter_block::format_line(fmt, raw, role);
            out.extend(fmt.wrap_line(formatted, width));
        }
    }
    buf.clear();
}
