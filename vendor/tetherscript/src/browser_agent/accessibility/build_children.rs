//! Child accessibility-node traversal.

use crate::browser::{Document, Element, Node};

use super::super::model::AccessibilityNode;
use super::{node, FocusMap};

pub(super) fn collect(
    document: &Document,
    value: &Element,
    path: &[usize],
    ancestors: &[Element],
    focus_map: &FocusMap,
) -> Vec<AccessibilityNode> {
    let mut next = ancestors.to_vec();
    next.push(value.clone());
    value
        .children
        .iter()
        .enumerate()
        .flat_map(|(index, child)| child_node(document, child, path, index, &next, focus_map))
        .collect()
}

fn child_node(
    document: &Document,
    child: &Node,
    path: &[usize],
    index: usize,
    ancestors: &[Element],
    focus_map: &FocusMap,
) -> Vec<AccessibilityNode> {
    let mut child_path = path.to_vec();
    child_path.push(index);
    node::node(document, child, child_path, ancestors, focus_map)
}
