//! Select option matching helpers.

use crate::browser::{self, Element, Node};

pub(crate) fn has(element: &Element, value: &str) -> bool {
    element
        .children
        .iter()
        .any(|child| value_for(child).is_some_and(|option| option == value))
}

fn value_for(node: &Node) -> Option<String> {
    let Node::Element(element) = node else {
        return None;
    };
    if !element.tag.eq_ignore_ascii_case("option") {
        return None;
    }
    Some(
        element
            .attrs
            .get("value")
            .cloned()
            .unwrap_or_else(|| browser::text_content(node)),
    )
}
