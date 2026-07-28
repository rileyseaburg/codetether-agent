//! DOM path lookup helpers for navigation actions.

use crate::browser::{Document, Element, Node};

pub(crate) fn element_at_path<'a>(document: &'a Document, path: &[usize]) -> Option<&'a Element> {
    let node = node_at_path(document, path)?;
    match node {
        Node::Element(element) => Some(element),
        Node::Text(_) => None,
    }
}

pub(crate) fn closest_form<'a>(
    document: &'a Document,
    path: &[usize],
) -> Option<(Vec<usize>, &'a Element)> {
    for len in (1..=path.len()).rev() {
        let prefix = path[..len].to_vec();
        let element = element_at_path(document, &prefix)?;
        if element.tag.eq_ignore_ascii_case("form") {
            return Some((prefix, element));
        }
    }
    None
}

fn node_at_path<'a>(document: &'a Document, path: &[usize]) -> Option<&'a Node> {
    let (first, rest) = path.split_first()?;
    let mut node = document.children.get(*first)?;
    for index in rest {
        let Node::Element(element) = node else {
            return None;
        };
        node = element.children.get(*index)?;
    }
    Some(node)
}
