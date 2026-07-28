//! DOM-level visibility predicates for selector extensions.

use crate::browser::Element;

pub(crate) fn visible(element: &Element, ancestors: &[Element]) -> bool {
    ancestors
        .iter()
        .chain(std::iter::once(element))
        .all(visible_self)
}

fn visible_self(element: &Element) -> bool {
    !element.attrs.contains_key("hidden")
        && !attr_true(element, "aria-hidden")
        && element
            .attrs
            .get("type")
            .is_none_or(|value| value != "hidden")
        && !style_hidden(element.attrs.get("style"))
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
        "opacity" => value == "0" || value == "0.0",
        _ => false,
    }
}

fn attr_true(element: &Element, name: &str) -> bool {
    element
        .attrs
        .get(name)
        .is_some_and(|value| value.eq_ignore_ascii_case("true"))
}
