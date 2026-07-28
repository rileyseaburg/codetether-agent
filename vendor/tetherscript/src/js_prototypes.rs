//! Shared JavaScript native prototype registry.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::JsValue;

thread_local! {
    static PROTOTYPES: RefCell<HashMap<&'static str, JsValue>> =
        RefCell::new(HashMap::new());
}

pub(super) fn reset() {
    PROTOTYPES.with(|prototypes| prototypes.borrow_mut().clear());
}

pub(super) fn empty() -> JsValue {
    JsValue::Object(Rc::new(RefCell::new(HashMap::new())))
}

pub(super) fn register(name: &'static str, prototype: JsValue) -> JsValue {
    PROTOTYPES.with(|prototypes| {
        prototypes.borrow_mut().insert(name, prototype.clone());
    });
    prototype
}

pub(super) fn get(name: &'static str) -> Option<JsValue> {
    PROTOTYPES.with(|prototypes| prototypes.borrow().get(name).cloned())
}

pub(super) fn property(name: &'static str, prop: &str) -> Option<JsValue> {
    PROTOTYPES.with(|prototypes| {
        let prototype = prototypes.borrow().get(name).cloned()?;
        match prototype {
            JsValue::Object(object) => object.borrow().get(prop).cloned(),
            _ => None,
        }
    })
}
