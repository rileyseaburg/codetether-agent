use std::collections::HashMap;

use super::super::super::*;
use super::super::{attr_update, handle_ref};

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    let handle = handle_ref::new(obj, handle);
    obj.insert(
        "toggleAttribute".into(),
        native("toggleAttribute", None, move |args| toggle(&handle, args)),
    );
}

fn toggle(handle: &handle_ref::HandleRef, args: &[JsValue]) -> Result<JsValue, String> {
    let handle = handle.current();
    let name = args.first().unwrap_or(&JsValue::Undefined).display();
    let next = args
        .get(1)
        .map(JsValue::truthy)
        .unwrap_or_else(|| attr_update::value(&handle, &name).is_none());
    if next {
        attr_update::set(&handle, &name, String::new())?;
    } else {
        attr_update::remove(&handle, &name)?;
    }
    Ok(JsValue::Bool(next))
}
