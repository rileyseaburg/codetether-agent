//! Shared DOM traversal for resource discovery.

use crate::browser::{Document, Element, Node};

pub(crate) fn elements(document: &Document, mut visit: impl FnMut(&Element)) {
    for child in &document.children {
        node(child, &mut visit);
    }
}

fn node(node: &Node, visit: &mut impl FnMut(&Element)) {
    if let Node::Element(element) = node {
        visit(element);
        for child in &element.children {
            self::node(child, visit);
        }
    }
}
