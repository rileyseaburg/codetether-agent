use std::cmp::Ordering;

pub(super) fn cmp_path(left: &[usize], right: &[usize]) -> Ordering {
    for (a, b) in left.iter().zip(right) {
        match a.cmp(b) {
            Ordering::Equal => {}
            other => return other,
        }
    }
    left.len().cmp(&right.len())
}
