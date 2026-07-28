//! Script element kind checks for resource inlining.

use crate::browser::Element;

pub(crate) fn external(element: &Element) -> bool {
    element.tag.eq_ignore_ascii_case("script")
        && element
            .attrs
            .get("src")
            .is_some_and(|value| !value.trim().is_empty())
}

pub(crate) fn module(element: &Element) -> bool {
    element
        .attrs
        .get("type")
        .is_some_and(|value| value.trim().eq_ignore_ascii_case("module"))
}
