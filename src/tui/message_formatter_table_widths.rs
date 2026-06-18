//! Column-width capping so a table fits the available terminal width.

/// Shrink the widest columns until the rendered width fits `max_width`.
///
/// Overhead is a 2-space indent plus a 2-space gap between each column.
pub(super) fn cap_widths(widths: &mut [usize], max_width: usize) {
    let overhead = 2 + widths.len().saturating_sub(1) * 2;
    let budget = max_width.saturating_sub(overhead).max(widths.len() * 3);
    while widths.iter().sum::<usize>() > budget {
        if let Some(max) = widths.iter_mut().filter(|x| **x > 3).max_by_key(|x| **x) {
            *max -= 1;
        } else {
            break;
        }
    }
}
