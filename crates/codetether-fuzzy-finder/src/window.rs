//! Fuzzy finder window presentation.

/// Scroll offset that keeps `selected` visible within a window of `rows`.
pub(crate) fn window_offset(selected: usize, rows: usize, total: usize) -> usize {
    if rows == 0 || total <= rows {
        return 0;
    }
    let max_offset = total - rows;
    selected.saturating_sub(rows - 1).min(max_offset)
}
