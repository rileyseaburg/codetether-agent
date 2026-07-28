//! Accessibility tree construction.

#[path = "build_children.rs"]
mod children;
#[path = "build_element.rs"]
mod element;
#[path = "build_node.rs"]
mod node;
#[path = "build_text.rs"]
mod text;

use std::collections::HashMap;

use crate::browser::Document;

use super::focus;
use super::model::{AccessibilityNode, AccessibilitySnapshot};

pub(super) type FocusMap = HashMap<Vec<usize>, usize>;

pub(super) fn snapshot(document: &Document) -> AccessibilitySnapshot {
    let entries = focus::order(document);
    let focus_order = entries.iter().map(|entry| entry.selector.clone()).collect();
    let focus_map = entries
        .iter()
        .enumerate()
        .map(|(index, entry)| (entry.path.clone(), index))
        .collect();
    AccessibilitySnapshot {
        roots: roots(document, &focus_map),
        focus_order,
    }
}

fn roots(document: &Document, focus_map: &FocusMap) -> Vec<AccessibilityNode> {
    document
        .children
        .iter()
        .enumerate()
        .flat_map(|(index, child)| node::node(document, child, vec![index], &[], focus_map))
        .collect()
}
