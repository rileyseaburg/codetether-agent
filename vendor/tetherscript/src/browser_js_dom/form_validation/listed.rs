use super::*;

pub(super) fn handles(form: &DomHandle) -> Vec<DomHandle> {
    let Some(Node::Element(el)) = form.node() else {
        return Vec::new();
    };
    let mut out = Vec::new();
    walk(form, &el, &mut out);
    out
}

fn walk(parent: &DomHandle, el: &Element, out: &mut Vec<DomHandle>) {
    for (index, child) in el.children.iter().enumerate() {
        let Node::Element(child_el) = child else {
            continue;
        };
        let handle = child_handle(parent, index);
        if controls::is_listed(child_el) {
            out.push(handle.clone());
        }
        walk(&handle, child_el, out);
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
