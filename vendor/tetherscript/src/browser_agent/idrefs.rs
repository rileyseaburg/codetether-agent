//! ID reference resolution for accessible names.

use crate::browser::{text_content, Document, Element, Node};
use crate::browser_agent::text_match::clean;

pub(crate) fn text_by_idrefs(document: &Document, idrefs: &str) -> Option<String> {
    let parts: Vec<String> = idrefs
        .split_whitespace()
        .filter_map(|id| text_by_id(&document.children, id))
        .collect();
    (!parts.is_empty()).then(|| clean(&parts.join(" ")))
}

fn text_by_id(nodes: &[Node], id: &str) -> Option<String> {
    for node in nodes {
        if let Node::Element(element) = node {
            if has_id(element, id) {
                return Some(clean(&text_content(node)));
            }
            if let Some(text) = text_by_id(&element.children, id) {
                return Some(text);
            }
        }
    }
    None
}

fn has_id(element: &Element, id: &str) -> bool {
    element.attrs.get("id").is_some_and(|value| value == id)
}
