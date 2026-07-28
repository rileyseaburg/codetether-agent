use super::super::*;

pub(super) fn tree(handle: &DomHandle) {
    if matches!(handle.node(), Some(Node::Element(el)) if el.tag == "select") {
        value_attr(handle);
    }
    if let Some(Node::Element(el)) = handle.node() {
        for index in 0..el.children.len() {
            let mut path = handle.path.clone();
            path.push(index);
            tree(&DomHandle {
                root: handle.root.clone(),
                path,
            });
        }
    }
}

pub(super) fn value_attr(select: &DomHandle) {
    let value = super::read::value(select);
    select.with_node_mut(|node| {
        if let Node::Element(el) = node {
            el.attrs.insert("value".into(), value);
        }
    });
}

pub(super) fn selected_attr(option: &DomHandle, selected: bool) {
    option.with_node_mut(|node| {
        if let Node::Element(el) = node {
            if selected {
                el.attrs.insert("selected".into(), String::new());
            } else {
                el.attrs.remove("selected");
            }
        }
    });
}
