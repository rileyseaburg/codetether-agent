//! Contiguous omitted ranges from a keep mask.

pub fn collect_dropped_ranges(keep: &[bool]) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut i = 0;
    while i < keep.len() {
        if !keep[i] {
            let start = i;
            while i < keep.len() && !keep[i] {
                i += 1;
            }
            ranges.push((start, i));
        } else {
            i += 1;
        }
    }
    ranges
}
