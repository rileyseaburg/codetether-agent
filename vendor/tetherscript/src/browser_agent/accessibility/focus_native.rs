//! Native focusability predicates for accessibility order.

use crate::browser::Element;

use super::super::visibility;

pub(super) fn focusable(element: &Element) -> bool {
    semantic(element) || editable(element)
}

pub(super) fn disabled(element: &Element) -> bool {
    element.attrs.contains_key("disabled") || visibility::attr_true(element, "aria-disabled")
}

fn semantic(element: &Element) -> bool {
    match element.tag.as_str() {
        "a" | "area" => element.attrs.contains_key("href"),
        "button" | "select" | "textarea" | "summary" => true,
        "input" => !visibility::hidden_subtree(element),
        _ => false,
    }
}

fn editable(element: &Element) -> bool {
    element
        .attrs
        .get("contenteditable")
        .is_some_and(|value| value != "false")
}
