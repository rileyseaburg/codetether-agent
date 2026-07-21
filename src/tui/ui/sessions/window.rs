//! Viewport bounds for large session collections.

use std::ops::Range;

pub(crate) fn bounds(selected: usize, total: usize, rows: usize) -> Range<usize> {
    if total == 0 || rows == 0 {
        return 0..0;
    }
    let selected = selected.min(total - 1);
    let start = selected
        .saturating_sub(rows / 2)
        .min(total.saturating_sub(rows));
    start..(start + rows).min(total)
}

#[cfg(test)]
mod tests {
    use super::bounds;

    #[test]
    fn keeps_selection_inside_a_bounded_window() {
        assert_eq!(bounds(50, 100, 9), 46..55);
        assert_eq!(bounds(99, 100, 9), 91..100);
    }

    #[test]
    fn handles_empty_or_short_lists() {
        assert_eq!(bounds(0, 0, 9), 0..0);
        assert_eq!(bounds(2, 3, 9), 0..3);
    }
}
