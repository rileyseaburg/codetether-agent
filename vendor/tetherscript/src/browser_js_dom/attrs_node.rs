use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::super::super::*;
use super::super::handle_ref;

pub(super) type AttrEntry = (String, String);

pub(super) fn entries(handle: &handle_ref::HandleRef) -> Vec<AttrEntry> {
    let mut entries = match handle.current().node() {
        Some(Node::Element(el)) => el.attrs.into_iter().collect(),
        _ => Vec::new(),
    };
    entries.sort_by(|left: &AttrEntry, right: &AttrEntry| left.0.cmp(&right.0));
    entries
}

pub(super) fn from_entry(handle: &handle_ref::HandleRef, entry: &AttrEntry) -> JsValue {
    let mut obj = HashMap::new();
    obj.insert("name".into(), JsValue::String(entry.0.clone()));
    obj.insert("nodeName".into(), JsValue::String(entry.0.clone()));
    obj.insert("nodeType".into(), JsValue::Number(2.0));
    obj.insert("value".into(), JsValue::String(entry.1.clone()));
    obj.insert("nodeValue".into(), JsValue::String(entry.1.clone()));
    obj.insert("specified".into(), JsValue::Bool(true));
    obj.insert(
        "ownerElement".into(),
        node_reference_object(handle.current()),
    );
    JsValue::Object(Rc::new(RefCell::new(obj)))
}
