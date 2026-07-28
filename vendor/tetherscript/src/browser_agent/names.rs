//! Accessible-name computation used by locators.

use crate::browser::{text_content, Document, Element, Node};
use crate::browser_agent::{idrefs, labels};

pub(crate) fn accessible_name(
    document: &Document,
    node: &Node,
    element: &Element,
    ancestors: &[Element],
) -> String {
    label_name(document, element, ancestors)
        .or_else(|| attr(element, "alt"))
        .or_else(|| text_name(node, element))
        .or_else(|| attr(element, "title"))
        .or_else(|| attr(element, "placeholder"))
        .unwrap_or_default()
}

pub(crate) fn label_name(
    document: &Document,
    element: &Element,
    ancestors: &[Element],
) -> Option<String> {
    attr(element, "aria-label")
        .or_else(|| labelledby(document, element))
        .or_else(|| labels::label_text(document, element, ancestors))
        .or_else(|| attr(element, "title"))
}

fn attr(element: &Element, name: &str) -> Option<String> {
    element
        .attrs
        .get(name)
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn labelledby(document: &Document, element: &Element) -> Option<String> {
    element
        .attrs
        .get("aria-labelledby")
        .and_then(|ids| idrefs::text_by_idrefs(document, ids))
}

fn text_name(node: &Node, element: &Element) -> Option<String> {
    names_from_content(element).then(|| text_content(node))
}

fn names_from_content(element: &Element) -> bool {
    matches!(
        element.tag.as_str(),
        "a" | "button" | "summary" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6"
    )
}
