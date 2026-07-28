use super::super::*;

pub(super) fn element_or_first(handle: &DomHandle, tag: &str) -> JsValue {
    find_by_selector(&handle.root, tag)
        .or_else(|| first_element_path(handle))
        .map(|path| ops::handle_object(handle.root.clone(), path))
        .unwrap_or(JsValue::Null)
}

pub(super) fn element_or_new(handle: &DomHandle, tag: &str) -> JsValue {
    find_by_selector(&handle.root, tag)
        .map(|path| ops::handle_object(handle.root.clone(), path))
        .unwrap_or_else(|| ops::detached_object(new_element(tag)))
}

fn first_element_path(handle: &DomHandle) -> Option<Vec<usize>> {
    let Node::Element(el) = &*handle.root.borrow() else {
        return None;
    };
    el.children
        .iter()
        .position(|node| matches!(node, Node::Element(_)))
        .map(|index| vec![index])
}

fn new_element(tag: &str) -> Node {
    Node::Element(Element {
        tag: tag.into(),
        attrs: HashMap::new(),
        children: Vec::new(),
    })
}
