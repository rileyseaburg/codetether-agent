use super::*;

pub(super) fn element(node: &Node) -> Option<&Element> {
    match node {
        Node::Element(el) => Some(el),
        Node::Text(_) => None,
    }
}

pub(super) fn is_control(el: &Element) -> bool {
    matches!(el.tag.as_str(), "input" | "textarea" | "select")
}

pub(super) fn is_listed(el: &Element) -> bool {
    matches!(el.tag.as_str(), "input" | "button" | "select" | "textarea")
}

pub(super) fn will_validate(el: &Element) -> bool {
    if !is_control(el) || el.attrs.contains_key("disabled") {
        return false;
    }
    if el.tag != "input" {
        return true;
    }
    !matches!(
        input_type(el).as_str(),
        "button" | "hidden" | "image" | "reset" | "submit"
    )
}

pub(super) fn input_type(el: &Element) -> String {
    el.attrs
        .get("type")
        .map(|ty| ty.to_ascii_lowercase())
        .unwrap_or_else(|| "text".into())
}
