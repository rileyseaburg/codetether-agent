use super::*;

pub(super) fn ancestor(range: &state::RangeState) -> DomHandle {
    let mut path = Vec::new();
    for (left, right) in range.start.handle.path.iter().zip(&range.end.handle.path) {
        if left != right {
            break;
        }
        path.push(*left);
    }
    DomHandle {
        root: range.start.handle.root.clone(),
        path,
    }
}
