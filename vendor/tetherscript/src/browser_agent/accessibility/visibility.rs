//! Accessibility subtree visibility predicates.

use crate::browser::Element;

pub(super) fn hidden_subtree(element: &Element) -> bool {
    element.attrs.contains_key("hidden")
        || element.attrs.contains_key("inert")
        || attr_true(element, "aria-hidden")
        || hidden_input(element)
        || style_hidden(element.attrs.get("style"))
}

fn hidden_input(element: &Element) -> bool {
    element.tag == "input"
        && element
            .attrs
            .get("type")
            .is_some_and(|value| value.eq_ignore_ascii_case("hidden"))
}

fn style_hidden(style: Option<&String>) -> bool {
    style.is_some_and(|style| style.split(';').any(hidden_declaration))
}

fn hidden_declaration(raw: &str) -> bool {
    let Some((name, value)) = raw.split_once(':') else {
        return false;
    };
    let name = name.trim().to_ascii_lowercase();
    let value = value.trim().to_ascii_lowercase();
    match name.as_str() {
        "display" => value == "none",
        "visibility" => matches!(value.as_str(), "hidden" | "collapse"),
        _ => false,
    }
}

pub(super) fn attr_true(element: &Element, name: &str) -> bool {
    element
        .attrs
        .get(name)
        .is_some_and(|value| value.eq_ignore_ascii_case("true"))
}
