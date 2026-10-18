//! Horizontal character-window scroll adjustment.
/// Compute the scroll needed to keep the cursor inside the visible width.
///
/// # Arguments
/// * `cursor` - Current character index.
/// * `scroll` - Current horizontal character offset.
/// * `visible_width` - Available character positions.
///
/// # Returns
/// Adjusted horizontal scroll offset.
///
/// # Examples
/// ```
/// assert_eq!(codetether_composer_view::cursor_visibility::ensure_input_cursor_visible(5, 0, 4), 2);
/// ```
pub fn ensure_input_cursor_visible(
    cursor: usize,
    mut scroll: usize,
    visible_width: usize,
) -> usize {
    if visible_width == 0 {
        return cursor;
    }
    if cursor < scroll {
        scroll = cursor;
    } else if cursor >= scroll.saturating_add(visible_width) {
        scroll = cursor.saturating_sub(visible_width.saturating_sub(1));
    }

    scroll
}
