use std::cmp::Ordering;

pub(super) fn id(left: &str, right: &str) -> Ordering {
    match (id_num(left), id_num(right)) {
        (Some(left), Some(right)) => left.cmp(&right),
        _ => left.cmp(right),
    }
}

pub(super) fn path(left: &[usize], right: &[usize]) -> Ordering {
    for (left, right) in left.iter().zip(right) {
        match left.cmp(right) {
            Ordering::Equal => {}
            order => return order,
        }
    }
    left.len().cmp(&right.len())
}

fn id_num(value: &str) -> Option<u64> {
    value.strip_prefix('h')?.parse().ok()
}
