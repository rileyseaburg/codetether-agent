use super::*;

pub(super) fn prop(element: &Element) -> String {
    normalize(element.attrs.get("contenteditable").map(String::as_str))
}

pub(super) fn effective(handle: &DomHandle) -> bool {
    if let Some(Node::Element(element)) = handle.node() {
        if let Some(editable) = state(&element) {
            return editable;
        }
    }
    for element in handle.ancestors().into_iter().rev() {
        if let Some(editable) = state(&element) {
            return editable;
        }
    }
    false
}

pub(super) fn set_value(raw: &str) -> Option<String> {
    match keyword(raw).as_str() {
        "inherit" => None,
        "" | "true" => Some("true".into()),
        "false" => Some("false".into()),
        "plaintext-only" => Some("plaintext-only".into()),
        _ => Some(raw.trim().into()),
    }
}

pub(super) fn state(element: &Element) -> Option<bool> {
    match prop(element).as_str() {
        "true" | "plaintext-only" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn normalize(value: Option<&str>) -> String {
    let Some(raw) = value else {
        return "inherit".into();
    };
    match keyword(raw).as_str() {
        "" | "true" => "true".into(),
        "false" => "false".into(),
        "inherit" => "inherit".into(),
        "plaintext-only" => "plaintext-only".into(),
        _ => raw.trim().into(),
    }
}

fn keyword(raw: &str) -> String {
    raw.trim().to_ascii_lowercase()
}
