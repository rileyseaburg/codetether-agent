use super::*;

pub(super) fn push(object: &JsObject, event_type: &str, listener: JsValue) {
    let key = key(event_type);
    let mut object = object.borrow_mut();
    let value = object
        .entry(key)
        .or_insert_with(|| JsValue::Array(Rc::new(RefCell::new(Vec::new()))));
    if let JsValue::Array(items) = value {
        items.borrow_mut().push(listener);
    }
}

pub(super) fn remove(object: &JsObject, event_type: &str, listener: &JsValue) {
    if let Some(JsValue::Array(items)) = object.borrow().get(&key(event_type)) {
        items.borrow_mut().retain(|item| item != listener);
    }
}

pub(super) fn get(object: &JsObject, event_type: &str) -> Vec<JsValue> {
    match object.borrow().get(&key(event_type)) {
        Some(JsValue::Array(items)) => items.borrow().clone(),
        _ => Vec::new(),
    }
}

fn key(event_type: &str) -> String {
    format!("__listeners:{event_type}")
}
