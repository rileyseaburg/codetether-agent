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
    object.borrow_mut().insert(
        "item".into(),
        native("NamedNodeMap.item", Some(1), move |args| {
            Ok(entries
                .get(index(args.first()))
                .map(|entry| attrs_node::from_entry(&handle, entry))
                .unwrap_or(JsValue::Null))
        }),
    );
}

fn index(value: Option<&JsValue>) -> usize {
    match value.unwrap_or(&JsValue::Undefined) {
        JsValue::Number(n) if n.is_finite() && *n >= 0.0 => n.trunc() as usize,
        other => other.display().parse().unwrap_or(usize::MAX),
    }
}
