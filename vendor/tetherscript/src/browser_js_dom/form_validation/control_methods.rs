use super::*;

pub(super) fn install(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &DomHandle) {
    for name in ["checkValidity", "reportValidity"] {
        let current = obj.clone();
        let h = handle.clone();
        obj.borrow_mut().insert(
            name.into(),
            native(name, Some(0), move |_| {
                refresh::write(&current, &h);
                Ok(JsValue::Bool(check::validity(&h).valid()))
            }),
        );
    }
    let current = obj.clone();
    let h = handle.clone();
    obj.borrow_mut().insert(
        "setCustomValidity".into(),
        native("setCustomValidity", Some(1), move |args| {
            let message = args.first().unwrap_or(&JsValue::Undefined).display();
            custom::set(&h, message);
            refresh::write(&current, &h);
            Ok(JsValue::Undefined)
        }),
    );
}
