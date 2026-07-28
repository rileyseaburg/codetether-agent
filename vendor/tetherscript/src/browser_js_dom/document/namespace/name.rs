use super::super::super::*;

pub(super) const HTML_NS: &str = "http://www.w3.org/1999/xhtml";

pub(super) struct QualifiedName {
    pub(super) name: String,
    pub(super) local: String,
    pub(super) prefix: Option<String>,
}

pub(super) fn parse(value: &JsValue) -> QualifiedName {
    let name = value.display();
    let (prefix, local) = split(&name);
    QualifiedName {
        name,
        local,
        prefix,
    }
}

pub(super) fn namespace_value(value: Option<&JsValue>) -> JsValue {
    match value.unwrap_or(&JsValue::Undefined) {
        JsValue::Null | JsValue::Undefined => JsValue::Null,
        other => JsValue::String(other.display()),
    }
}

pub(super) fn node_name(namespace: &JsValue, name: &str) -> String {
    if matches!(namespace, JsValue::String(ns) if ns == HTML_NS) {
        name.to_ascii_uppercase()
    } else {
        name.into()
    }
}

pub(super) fn prefix_value(name: &QualifiedName) -> JsValue {
    name.prefix
        .as_ref()
        .map(|prefix| JsValue::String(prefix.clone()))
        .unwrap_or(JsValue::Null)
}

fn split(name: &str) -> (Option<String>, String) {
    match name.split_once(':') {
        Some((prefix, local)) => (Some(prefix.into()), local.into()),
        None => (None, name.into()),
    }
}
