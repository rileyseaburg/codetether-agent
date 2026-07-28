use super::super::*;

pub(super) fn all(select: &DomHandle) -> Vec<DomHandle> {
    let Some(Node::Element(el)) = select.node() else {
        return Vec::new();
    };
    let mut out = Vec::new();
    walk(select, &el, &mut out);
    out
}

fn walk(parent: &DomHandle, el: &Element, out: &mut Vec<DomHandle>) {
    for (index, child) in el.children.iter().enumerate() {
        let Node::Element(child_el) = child else {
            continue;
        };
        let handle = child_handle(parent, index);
        if child_el.tag == "option" {
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
