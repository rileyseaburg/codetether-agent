//! Element state predicates for selector extensions.

use crate::browser::Element;

pub(crate) fn enabled(element: &Element) -> bool {
    !disabled(element)
}

pub(crate) fn disabled(element: &Element) -> bool {
    element.attrs.contains_key("disabled") || attr_true(element, "aria-disabled")
}

pub(crate) fn checked(element: &Element) -> bool {
    element.attrs.contains_key("checked")
        || element.attrs.contains_key("selected")
        || attr_true(element, "aria-checked")
}

fn attr_true(element: &Element, name: &str) -> bool {
    element
        .attrs
        .get(name)
        .is_some_and(|value| value.eq_ignore_ascii_case("true"))
}
