//! Select control successful option extraction.

use crate::browser::{self, Element, Node};

pub(crate) fn values(element: &Element, name: String) -> Vec<(String, String)> {
    element
        .children
        .iter()
        .filter_map(|child| option(child, &name))
        .collect()
}

fn option(node: &Node, name: &str) -> Option<(String, String)> {
    let Node::Element(element) = node else {
        return None;
    };
    if !element.tag.eq_ignore_ascii_case("option") || !element.attrs.contains_key("selected") {
        return None;
    }
    Some((
        name.into(),
        element
            .attrs
            .get("value")
            .cloned()
            .unwrap_or_else(|| browser::text_content(node)),
    ))
}
