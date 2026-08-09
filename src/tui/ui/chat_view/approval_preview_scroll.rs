//! Wrapped-row scroll bounds for the approval preview.
//!
//! [`Paragraph`] scrolls by rendered rows, not logical lines. A single long
//! command wraps to many rows, so bounding the offset by `lines().count()`
//! pins it at zero and makes the tail unreachable. Row height is measured
//! against the wrap width instead.

/// Rows a text block occupies once wrapped to `width` columns.
pub(super) fn wrapped_rows(content: &str, width: u16) -> usize {
    let width = width.max(1) as usize;
    content
        .lines()
        .map(|line| row_span(line, width))
        .sum::<usize>()
        .max(1)
}

/// Highest scroll offset that still shows content in a `height`-row viewport.
pub(super) fn max_offset(content: &str, width: u16, height: u16) -> u16 {
    let rows = wrapped_rows(content, width);
    let visible = height.max(1) as usize;
    u16::try_from(rows.saturating_sub(visible)).unwrap_or(u16::MAX)
}

/// Rows one logical line occupies, counting display width.
fn row_span(line: &str, width: usize) -> usize {
    let columns = line.chars().count();
    columns.div_ceil(width).max(1)
}

#[cfg(test)]
#[path = "approval_preview_scroll_tests.rs"]
mod tests;
