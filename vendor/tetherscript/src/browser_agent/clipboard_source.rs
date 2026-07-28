//! Text extraction for clipboard copy actions.

use crate::browser::{self, Element, Node};

pub(crate) fn text(element: &Element) -> String {
    if element.tag.eq_ignore_ascii_case("input") {
        return element.attrs.get("value").cloned().unwrap_or_default();
    }
    browser::text_content(&Node::Element(element.clone()))
}
