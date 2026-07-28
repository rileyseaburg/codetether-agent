use std::collections::HashMap;

use super::super::super::*;
use super::super::handle_ref;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    let handle = handle_ref::new(obj, handle);
    method(
        obj,
        "getAttributeNS",
        2,
        handle.clone(),
        super::attrs_namespace_methods::get,
    );
    method(
        obj,
        "setAttributeNS",
        3,
        handle.clone(),
        super::attrs_namespace_methods::set,
    );
    method(
        obj,
        "hasAttributeNS",
        2,
        handle.clone(),
        super::attrs_namespace_methods::has,
    );
    method(
        obj,
        "removeAttributeNS",
        2,
        handle,
        super::attrs_namespace_methods::remove,
    );
}

fn method(
    obj: &mut HashMap<String, JsValue>,
    name: &str,
    arity: usize,
    handle: handle_ref::HandleRef,
    f: fn(&DomHandle, &[JsValue]) -> Result<JsValue, String>,
) {
    obj.insert(
        name.into(),
        native(name, Some(arity), move |args| f(&handle.current(), args)),
    );
}
