use super::*;
use std::cell::RefCell;
use std::rc::Rc;

pub(super) fn empty_object() -> JsValue {
    JsValue::Object(Rc::new(RefCell::new(HashMap::new())))
}

pub(super) fn snapshot(value: &JsValue) -> JsValue {
    match value {
        JsValue::Array(items) => JsValue::Array(Rc::new(RefCell::new(items.borrow().clone()))),
        JsValue::Object(object) => JsValue::Object(Rc::new(RefCell::new(object.borrow().clone()))),
        other => other.clone(),
    }
}
