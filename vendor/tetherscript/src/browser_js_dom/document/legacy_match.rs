use super::super::super::*;

pub(super) fn tag(el: &Element, tag: &str) -> bool {
    real(el) && (tag == "*" || el.tag.eq_ignore_ascii_case(tag))
}

pub(super) fn classes(el: &Element, want: &[&str]) -> bool {
    real(el) && !want.is_empty() && want.iter().all(|name| has_class(el, name))
}

pub(super) fn name(el: &Element, name: &str) -> bool {
    real(el) && el.attrs.get("name").is_some_and(|value| value == name)
}

pub(super) fn tokens(value: &str) -> Vec<&str> {
    value.split_whitespace().collect()
}

fn has_class(el: &Element, name: &str) -> bool {
    el.attrs
        .get("class")
        .is_some_and(|value| tokens(value).contains(&name))
}

fn real(el: &Element) -> bool {
    !el.tag.starts_with('#')
}
