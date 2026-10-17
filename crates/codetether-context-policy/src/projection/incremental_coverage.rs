//! Compute uncovered ranges from the final [`MessageOrigin`] sidecar.

use super::incremental_types::MessageOrigin;

/// Build a contiguous `(start, end)` list of every original index in
/// `0..origin_len` that no surviving message in `origins` covers.
///
/// * [`MessageOrigin::Clone(idx)`] covers `idx`.
/// * [`MessageOrigin::Summary(range)`] covers `range.start..range.end`.
/// * [`MessageOrigin::Synthetic`] covers nothing.
pub fn uncovered_ranges(origins: &[MessageOrigin], origin_len: usize) -> Vec<(usize, usize)> {
    let mut covered = vec![false; origin_len];
    for origin in origins {
        match origin {
            MessageOrigin::Clone(idx) if *idx < origin_len => covered[*idx] = true,
            MessageOrigin::Summary(range) => {
                let end = range.end.min(origin_len);
                let start = range.start.min(end);
                covered[start..end].fill(true);
            }
            _ => {}
        }
    }
    let mut out = Vec::new();
    let mut i = 0;
    while i < origin_len {
        if !covered[i] {
            let start = i;
            while i < origin_len && !covered[i] {
                i += 1;
            }
            out.push((start, i));
        } else {
            i += 1;
        }
    }
    out
}
