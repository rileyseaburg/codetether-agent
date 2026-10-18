//! Measure natural column widths and apply the shared width cap.
use super::widths_mod;
use crate::util::w;

/// Compute display widths per column, capped so the table fits `max_width`.
pub(super) fn column_widths(
    header: &[String],
    body: &[Vec<String>],
    cols: usize,
    max_width: usize,
) -> Vec<usize> {
    let mut widths = vec![0usize; cols];
    let mut scan = |row: &[String]| {
        for (i, cell) in row.iter().enumerate() {
            if i < cols {
                widths[i] = widths[i].max(w(cell)).max(3);
            }
        }
    };
    scan(header);
    for row in body {
        scan(row);
    }
    widths_mod::cap_widths(&mut widths, max_width);
    widths
}
