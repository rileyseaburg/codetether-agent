use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::super::super::*;
use super::super::handle_ref;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    let handle = handle_ref::new(obj, handle);
    obj.insert(
        "getAttributeNames".into(),
        native("getAttributeNames", Some(0), move |_| Ok(names(&handle))),
    );
}

fn names(handle: &handle_ref::HandleRef) -> JsValue {
    let mut names = match handle.current().node() {
        Some(Node::Element(el)) => el.attrs.keys().cloned().collect::<Vec<_>>(),
        _ => Vec::new(),
    };
    names.sort();
    JsValue::Array(Rc::new(RefCell::new(
        names.into_iter().map(JsValue::String).collect(),
    )))
}
