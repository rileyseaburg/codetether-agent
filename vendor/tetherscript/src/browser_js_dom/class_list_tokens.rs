use super::*;

pub(super) fn current(handle: &DomHandle) -> Vec<String> {
    match handle.node() {
        Some(Node::Element(el)) => el
            .attrs
            .get("class")
            .map(|value| parse(value))
            .unwrap_or_default(),
        _ => Vec::new(),
    }
}

pub(super) fn parse(value: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    for token in value.split_whitespace() {
        if !tokens.iter().any(|item: &String| item == token) {
            tokens.push(token.to_string());
        }
    }
    tokens
}

pub(super) fn set(handle: &DomHandle, tokens: Vec<String>) {
    handle.with_node_mut(|node| {
        if let Node::Element(el) = node {
            if tokens.is_empty() {
                el.attrs.remove("class");
            } else {
                el.attrs.insert("class".into(), tokens.join(" "));
            }
        }
    });
}

pub(super) fn index(value: Option<&JsValue>) -> Option<usize> {
    let index = match value.unwrap_or(&JsValue::Undefined) {
        JsValue::Number(number) => *number,
        JsValue::String(text) => text.trim().parse().unwrap_or(f64::NAN),
        JsValue::Bool(true) => 1.0,
        JsValue::Bool(false) | JsValue::Null => 0.0,
        _ => f64::NAN,
    };
    (index.is_finite() && index >= 0.0).then_some(index.trunc() as usize)
}
