use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::super::super::*;
use super::super::handle_ref;
use super::attrs_node::{self, AttrEntry};

pub(super) fn install(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    handle: handle_ref::HandleRef,
    entries: Vec<AttrEntry>,
) {
    let weak = Rc::downgrade(object);
    object.borrow_mut().insert(
        "forEach".into(),
        native("NamedNodeMap.forEach", None, move |args| {
            let callback = args
                .first()
                .cloned()
                .ok_or_else(|| "NamedNodeMap.forEach: expected callback".to_string())?;
            let this_arg = args.get(1).cloned().unwrap_or(JsValue::Undefined);
            let map = weak
                .upgrade()
                .map(JsValue::Object)
                .unwrap_or(JsValue::Undefined);
            for (index, entry) in entries.iter().enumerate() {
                js::call_function_with_this(
                    callback.clone(),
                    this_arg.clone(),
                    &[
                        attrs_node::from_entry(&handle, entry),
                        JsValue::Number(index as f64),
                        map.clone(),
                    ],
                )?;
            }
            Ok(JsValue::Undefined)
        }),
    );
}
