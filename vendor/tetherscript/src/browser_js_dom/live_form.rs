//! Live form-control property getters.

use super::*;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    let h = handle.clone();
    obj.insert(
        "__get:value".into(),
        native("get_value", Some(0), move |_| {
            Ok(JsValue::String(h.input_value()))
        }),
    );
    let h = handle.clone();
    obj.insert(
        "__get:checked".into(),
        native("get_checked", Some(0), move |_| {
            Ok(JsValue::Bool(checked(&h)))
        }),
    );
}

fn checked(handle: &DomHandle) -> bool {
    matches!(handle.node(), Some(Node::Element(el)) if el.attrs.contains_key("checked"))
}
