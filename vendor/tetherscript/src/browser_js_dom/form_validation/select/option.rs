use super::super::*;

pub(super) fn install(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &DomHandle) {
    write(obj, handle);
    let current = obj.clone();
    let h = handle.clone();
    obj.borrow_mut().insert(
        "__set:selected".into(),
        native("set_option_selected", Some(1), move |args| {
            let selected = args.first().unwrap_or(&JsValue::Undefined).truthy();
            super::mutation::set_option_selected(&h, selected);
            write(&current, &h);
            Ok(JsValue::Bool(super::value::selected(&h)))
        }),
    );
}

fn write(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &DomHandle) {
    let mut obj = obj.borrow_mut();
    obj.insert("value".into(), JsValue::String(super::value::get(handle)));
    obj.insert("text".into(), JsValue::String(super::value::text(handle)));
    obj.insert(
        "selected".into(),
        JsValue::Bool(super::value::selected(handle)),
    );
    obj.insert(
        "index".into(),
        JsValue::Number(super::owner::index(handle) as f64),
    );
}
