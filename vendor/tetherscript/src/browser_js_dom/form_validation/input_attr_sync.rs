use super::*;

pub(super) fn value(obj: &Rc<RefCell<HashMap<String, JsValue>>>, value: &str) {
    obj.borrow_mut()
        .insert("value".into(), JsValue::String(value.into()));
}

pub(super) fn checked(obj: &Rc<RefCell<HashMap<String, JsValue>>>, checked: bool) {
    obj.borrow_mut()
        .insert("checked".into(), JsValue::Bool(checked));
}
