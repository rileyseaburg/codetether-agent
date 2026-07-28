use super::*;

pub(super) fn control(handle: &DomHandle) -> String {
    match handle.node() {
        Some(Node::Element(el)) if el.tag == "select" => select::value(handle),
        Some(Node::Element(_)) => handle.input_value(),
        _ => String::new(),
    }
}
