//! Editability helpers for fill actions.

use crate::browser::Element;

pub(crate) fn editable(element: &Element) -> bool {
    if element
        .attrs
        .get("contenteditable")
        .is_some_and(|v| v != "false")
    {
        return true;
    }
    if element.tag.eq_ignore_ascii_case("textarea") {
        return true;
    }
    element.tag.eq_ignore_ascii_case("input") && !blocked_input_type(element)
}

fn blocked_input_type(element: &Element) -> bool {
    let value = element.attrs.get("type").map(|v| v.to_ascii_lowercase());
    matches!(
        value.as_deref(),
        Some("button" | "checkbox" | "file" | "hidden" | "image" | "radio" | "reset" | "submit")
    )
}
