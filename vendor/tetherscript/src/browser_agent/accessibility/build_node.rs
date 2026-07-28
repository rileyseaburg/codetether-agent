//! DOM-node accessibility tree conversion.

use crate::browser::{Document, Element, Node};
use crate::browser_agent::roles;

use super::super::model::AccessibilityNode;
use super::super::visibility;
use super::{children, element, text, FocusMap};

pub(super) fn node(
    document: &Document,
    item: &Node,
    path: Vec<usize>,
    ancestors: &[Element],
    focus_map: &FocusMap,
) -> Vec<AccessibilityNode> {
    match item {
        Node::Text(value) => text::node(value, path),
        Node::Element(value) => element_node(document, item, value, path, ancestors, focus_map),
    }
}

fn element_node(
    document: &Document,
    item: &Node,
    value: &Element,
    path: Vec<usize>,
    ancestors: &[Element],
    focus_map: &FocusMap,
) -> Vec<AccessibilityNode> {
    if visibility::hidden_subtree(value) {
        return Vec::new();
    }
    let child_nodes = children::collect(document, value, &path, ancestors, focus_map);
    let role = roles::role_of(value);
    if matches!(role.as_str(), "none" | "presentation") {
        return child_nodes;
    }
    vec![element::entry(
        document,
        item,
        value,
        path,
        ancestors,
        focus_map,
        child_nodes,
    )]
}
