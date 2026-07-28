use super::super::*;

pub(super) fn get(handle: &DomHandle) -> String {
    match handle.node() {
        Some(Node::Element(el)) => from_element(&el),
        _ => String::new(),
    }
}

pub(super) fn text(handle: &DomHandle) -> String {
    match handle.node() {
        Some(Node::Element(el)) => browser::text_content(&Node::Element(el)),
        _ => String::new(),
    }
}

pub(super) fn selected(handle: &DomHandle) -> bool {
    matches!(handle.node(), Some(Node::Element(el)) if el.attrs.contains_key("selected"))
}

fn from_element(el: &Element) -> String {
    el.attrs
        .get("value")
        .cloned()
        .unwrap_or_else(|| browser::text_content(&Node::Element(el.clone())))
}
