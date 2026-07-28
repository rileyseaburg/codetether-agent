//! Canvas dimension attributes.

use super::*;

pub(super) fn dimension_value(value: Option<&JsValue>) -> u32 {
    match value.unwrap_or(&JsValue::Undefined) {
        JsValue::Number(n) if n.is_finite() && *n >= 0.0 => *n as u32,
        other => other.display().parse().unwrap_or(300),
    }
}

pub(super) fn dimensions(handle: &DomHandle) -> (u32, u32) {
    let Some(Node::Element(el)) = handle.node() else {
        return (300, 150);
    };
    (
        el.attrs
            .get("width")
            .and_then(|v| v.parse().ok())
            .unwrap_or(300),
        el.attrs
            .get("height")
            .and_then(|v| v.parse().ok())
            .unwrap_or(150),
    )
}

pub(super) fn is_dimension_attr(handle: &DomHandle, name: &str) -> bool {
    matches!(name, "width" | "height")
        && matches!(handle.node(), Some(Node::Element(el)) if el.tag == "canvas")
}

pub(super) fn set_dimension(handle: &DomHandle, name: &str, value: u32) {
    handle.with_node_mut(|node| {
        if let Node::Element(el) = node {
            el.attrs.insert(name.into(), value.to_string());
        }
    });
    super::store::reset_surface(handle);
}
