//! Visible-row window for the file picker list.

pub fn bounds(selected: usize, len: usize, rows: usize) -> (usize, usize) {
    if len == 0 || rows == 0 {
        return (0, 0);
    }
    let rows = rows.min(len);
    let half = rows / 2;
    let max_start = len.saturating_sub(rows);
    let start = selected.saturating_sub(half).min(max_start);
    (start, start + rows)
}

#[cfg(test)]
mod tests {
    use super::bounds;

    #[test]
    fn keeps_selected_visible_near_end() {
        assert_eq!(bounds(98, 100, 10), (90, 100));
    }

    #[test]
    fn centers_selected_when_possible() {
        assert_eq!(bounds(50, 100, 11), (45, 56));
    }
}
