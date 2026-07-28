use super::*;

pub(super) fn valid(handle: &DomHandle) -> bool {
    let Some(Node::Element(el)) = handle.node() else {
        return true;
    };
    let mut valid = true;
    walk(handle, &el, &mut valid);
    valid
}

fn walk(parent: &DomHandle, el: &Element, valid: &mut bool) {
    for (index, child) in el.children.iter().enumerate() {
        let Node::Element(child_el) = child else {
            continue;
        };
        let handle = child_handle(parent, index);
        if controls::is_control(child_el) && !check::validity(&handle).valid() {
            *valid = false;
        }
        walk(&handle, child_el, valid);
    }
}

fn child_handle(parent: &DomHandle, index: usize) -> DomHandle {
    let mut path = parent.path.clone();
    path.push(index);
    DomHandle {
        root: parent.root.clone(),
        path,
    }
}
