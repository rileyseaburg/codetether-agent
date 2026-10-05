//! Scroll bounds measured using the renderer's word wrapping and Unicode widths.

#[cfg(test)]
/// Rows a text block occupies using the same word wrapping as the renderer.
pub(super) fn wrapped_rows(content: &str, width: u16) -> usize {
    ratatui::widgets::Paragraph::new(content)
        .wrap(ratatui::widgets::Wrap { trim: false })
        .line_count(width.max(1))
        .max(1)
}

#[cfg(test)]
/// Highest scroll offset that still shows content in a height-row viewport.
pub(super) fn max_offset(content: &str, width: u16, height: u16) -> u16 {
    offset_for_rows(wrapped_rows(content, width), height)
}

/// Bound the offset by rendered rows, including word wrapping and Unicode.
pub(super) fn offset_for_rows(rows: usize, height: u16) -> u16 {
    u16::try_from(rows.saturating_sub(usize::from(height.max(1)))).unwrap_or(u16::MAX)
}

#[cfg(test)]
#[path = "approval_preview_scroll_tests.rs"]
mod tests;
