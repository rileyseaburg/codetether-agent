use super::*;

pub(super) fn node_extent(handle: &DomHandle) -> usize {
    match handle.node() {
        Some(Node::Text(text)) => text.chars().count(),
        Some(Node::Element(element)) => element.children.len(),
        None => 0,
    }
}
