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
        "getNamedItem".into(),
        native("NamedNodeMap.getNamedItem", Some(1), move |args| {
            let name = args.first().unwrap_or(&JsValue::Undefined).display();
            Ok(entries
                .iter()
                .find(|entry| entry.0 == name)
                .map(|entry| attrs_node::from_entry(&handle, entry))
                .unwrap_or(JsValue::Null))
        }),
    );
}
