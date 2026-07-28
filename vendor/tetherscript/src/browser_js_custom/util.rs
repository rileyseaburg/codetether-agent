use super::*;

pub(super) fn valid_name(name: &str) -> bool {
    name.contains('-') && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
}

pub(super) fn tag_name(node: &Node) -> Option<String> {
    match node {
        Node::Element(el) if !el.tag.starts_with('#') => Some(el.tag.clone()),
        _ => None,
    }
}

pub(super) fn object_prop(value: &JsValue, name: &str) -> Option<JsValue> {
    match value {
        JsValue::Object(obj) => obj.borrow().get(name).cloned(),
        _ => None,
    }
}

pub(super) fn callback(definition: &JsValue, name: &str) -> Option<JsValue> {
    if name == "constructor" && callable(definition) {
        return Some(definition.clone());
    }
    object_prop(definition, name).filter(callable)
}

pub(super) fn callable(value: &JsValue) -> bool {
    matches!(
        value,
        JsValue::Function(_) | JsValue::BoundFunction(_) | JsValue::Native(_)
    )
}

pub(super) fn observed_attributes(value: &JsValue) -> Vec<String> {
    let Some(JsValue::Array(items)) = object_prop(value, "observedAttributes") else {
        return Vec::new();
    };
    let items = items.borrow();
    items
        .iter()
        .map(JsValue::display)
        .filter(|item| !item.is_empty())
        .collect()
}

pub(super) fn js_option(value: Option<String>) -> JsValue {
    value.map(JsValue::String).unwrap_or(JsValue::Null)
}
