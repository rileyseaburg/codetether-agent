//! Bridge shared Markdown table rendering to native prose fallback.
use super::MessageFormatter;
pub use codetether_markdown_table::{is_table_row, render_table};
use ratatui::text::Line;

/// Flush buffered table rows into `out`.
///
/// Renders a valid GFM table as a bordered box, otherwise falls back to
/// plain per-line formatting. Clears `buf` when done.
pub(crate) fn flush(
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
            let formatted = crate::block::format_line(fmt, raw, role);
            out.extend(fmt.wrap_line(formatted, width));
        }
    }
    buf.clear();
}
#[cfg(test)]
#[path = "table_tests.rs"]
mod tests;
