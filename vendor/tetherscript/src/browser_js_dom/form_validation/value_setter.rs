use super::*;

pub(super) fn install(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &DomHandle) {
    let current = obj.clone();
    let h = handle.clone();
    obj.borrow_mut().insert(
        "__set:value".into(),
        native("set_validated_value", Some(1), move |args| {
            let value = args.first().unwrap_or(&JsValue::Undefined).display();
            h.set_input_value(value);
            refresh::write(&current, &h);
            input_number::refresh(&current, &h);
            Ok(JsValue::Undefined)
        }),
    );
}
