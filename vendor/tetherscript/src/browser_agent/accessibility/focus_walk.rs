//! DOM traversal for focus-order collection.

use crate::browser::{Document, Node};
use crate::browser_agent::interact::focus::selector_for;

use super::super::visibility;
use super::{focusable, tab_index};

pub(super) fn collect(document: &Document) -> Vec<(usize, i32, Vec<usize>, String)> {
    let mut out = Vec::new();
    for (index, child) in document.children.iter().enumerate() {
        visit(child, &[index], &mut out);
    }
    out
}

fn visit(node: &Node, path: &[usize], out: &mut Vec<(usize, i32, Vec<usize>, String)>) {
    let Node::Element(element) = node else { return };
    if visibility::hidden_subtree(element) {
        return;
    }
    let dom_index = out.len();
    if focusable(element) {
        out.push((
            dom_index,
            tab_index(element),
            path.to_vec(),
            selector_for(path, element),
        ));
    }
    for (index, child) in element.children.iter().enumerate() {
        let mut child_path = path.to_vec();
        child_path.push(index);
        visit(child, &child_path, out);
    }
}
