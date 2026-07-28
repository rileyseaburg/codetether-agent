use super::*;

pub(super) fn inserted(parent: &DomHandle, index: usize, count: usize) {
    if count > 0 {
        adjust(parent, index, count as isize, true);
    }
}

pub(super) fn replaced(parent: &DomHandle, index: usize, added: usize) {
    let delta = added as isize - 1;
    if delta != 0 {
        adjust(parent, index, delta, false);
    }
}

fn adjust(parent: &DomHandle, index: usize, delta: isize, include_index: bool) {
    DOM_HANDLE_REGISTRY.with(|registry| {
        for handle in registry.borrow_mut().values_mut() {
            let depth = parent.path.len();
            if !Rc::ptr_eq(&parent.root, &handle.root) {
                continue;
            }
            if handle.path.len() <= depth || !handle.path.starts_with(&parent.path) {
                continue;
            }
            let slot = &mut handle.path[depth];
            if *slot > index || (include_index && *slot >= index) {
                *slot = ((*slot as isize) + delta).max(0) as usize;
            }
        }
    });
}
