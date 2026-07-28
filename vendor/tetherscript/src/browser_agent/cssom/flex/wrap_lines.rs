//! Multi-line flex wrapping.

use super::{resolve, size, types::*};

/// Split items into flex lines based on the wrapping mode.
/// Returns a list of (start_index, end_index, cross_size) tuples.
pub fn lines(
    items: &mut [FlexItem],
    row: bool,
    main: i64,
    flex_wrap: FlexWrap,
) -> Vec<(usize, usize, i64)> {
    if matches!(flex_wrap, FlexWrap::NoWrap) || items.is_empty() {
        size::size_line(items, row, main);
        let c = items
            .iter()
            .map(|i| resolve::cross_size(i, row))
            .max()
            .unwrap_or(0);
        return vec![(0, items.len(), c)];
    }
    let mut out = Vec::new();
    let mut s = 0;
    let mut used = 0;
    let mut cross = 0;
    for i in 0..items.len() {
        let b = items[i]
            .flex_basis
            .unwrap_or_else(|| resolve::main_size(&items[i], row));
        if i > s && used + b > main {
            size::size_line(&mut items[s..i], row, main);
            out.push((s, i, cross));
            s = i;
            used = 0;
            cross = 0;
        }
        used += b;
        cross = cross.max(resolve::cross_size(&items[i], row));
    }
    size::size_line(&mut items[s..], row, main);
    cross = items[s..]
        .iter()
        .map(|i| resolve::cross_size(i, row))
        .max()
        .unwrap_or(0);
    out.push((s, items.len(), cross));
    if matches!(flex_wrap, FlexWrap::WrapReverse) {
        out.reverse();
    }
    out
}
