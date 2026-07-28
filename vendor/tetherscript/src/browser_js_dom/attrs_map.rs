use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::super::super::*;
use super::super::handle_ref;
use super::{attrs_each, attrs_item, attrs_named, attrs_node};

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    let handle = handle_ref::new(obj, handle);
    if is_real_element(&handle) {
        obj.insert("attributes".into(), object(handle));
    }
}

pub(super) fn object(handle: handle_ref::HandleRef) -> JsValue {
    let entries = attrs_node::entries(&handle);
    let mut obj = HashMap::new();
    obj.insert("length".into(), JsValue::Number(entries.len() as f64));
    for (index, entry) in entries.iter().enumerate() {
        obj.insert(index.to_string(), attrs_node::from_entry(&handle, entry));
    }
    let object = Rc::new(RefCell::new(obj));
    attrs_item::install(&object, handle.clone(), entries.clone());
    attrs_named::install(&object, handle.clone(), entries.clone());
    attrs_each::install(&object, handle, entries);
    JsValue::Object(object)
}

fn is_real_element(handle: &handle_ref::HandleRef) -> bool {
    matches!(handle.current().node(), Some(Node::Element(el)) if !el.tag.starts_with('#'))
}
