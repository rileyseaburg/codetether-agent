use super::*;

pub(super) fn install(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &DomHandle) {
    for name in ["checkValidity", "reportValidity"] {
        let h = handle.clone();
        obj.borrow_mut().insert(
            name.into(),
            native(name, Some(0), move |_| Ok(JsValue::Bool(form::valid(&h)))),
        );
    }
}
