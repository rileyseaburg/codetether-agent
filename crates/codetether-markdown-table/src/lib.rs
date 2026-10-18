//! GitHub-flavored Markdown table detection and terminal rendering.
//!
//! [is_table_row] identifies candidate rows, [is_separator_row] validates
//! separators, and [render_table] returns aligned styled lines for valid input.
//! Invalid input remains the caller's responsibility to format as prose.
use ratatui::text::Line;
mod parse;
mod render;
mod util;
pub use parse::{is_separator_row, is_table_row};

/// Render buffered candidate lines as a table when valid.
///
/// Returns `Some(lines)` when `buf` forms a valid GFM table (header row,
/// separator, optional body), otherwise `None` so the caller can fall back
/// to plain rendering.
/// # Arguments
/// * `buf` - Candidate rows in source order.
/// * `max_width` - Terminal width available for the table.
/// # Returns
/// Styled lines for a valid table, otherwise None.
/// # Examples
/// ```
/// assert!(codetether_markdown_table::render_table(&[], 80).is_none());
/// ```
pub fn render_table(buf: &[String], max_width: usize) -> Option<Vec<Line<'static>>> {
    if buf.len() < 2 || !is_separator_row(&buf[1]) {
        return None;
    }
    render::table(buf, max_width)
}
#[cfg(test)]
mod feature_tests;
#[cfg(test)]
mod tests;
