//! Common native and ARIA state extraction.

use crate::browser::Element;

use super::model::AccessibilityState;
use super::visibility;

pub(super) fn of(element: &Element) -> AccessibilityState {
    AccessibilityState {
        checked: token(element, "aria-checked").or_else(|| native_bool(element, "checked")),
        disabled: element.attrs.contains_key("disabled")
            || visibility::attr_true(element, "aria-disabled"),
        expanded: token(element, "aria-expanded"),
        selected: token(element, "aria-selected").or_else(|| native_bool(element, "selected")),
        pressed: token(element, "aria-pressed"),
    }
}

fn token(element: &Element, name: &str) -> Option<String> {
    element
        .attrs
        .get(name)
        .map(|value| value.trim().to_ascii_lowercase())
        .filter(|value| matches!(value.as_str(), "true" | "false" | "mixed"))
}

fn native_bool(element: &Element, attr: &str) -> Option<String> {
    native_applies(element, attr).then(|| element.attrs.contains_key(attr).to_string())
}

fn native_applies(element: &Element, attr: &str) -> bool {
    match attr {
        "checked" => native_checked(element),
        "selected" => element.tag == "option",
        _ => false,
    }
}

fn native_checked(element: &Element) -> bool {
    if element.tag != "input" {
        return false;
    }
    element
        .attrs
        .get("type")
        .is_some_and(|kind| matches!(kind.to_ascii_lowercase().as_str(), "checkbox" | "radio"))
}
