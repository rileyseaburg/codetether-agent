use super::super::*;

pub(super) fn install(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &DomHandle) {
    let current = obj.clone();
    let h = handle.clone();
    obj.borrow_mut().insert(
        "__set:selectedIndex".into(),
        native("set_selectedIndex", Some(1), move |args| {
            super::mutation::set_index(&h, index_arg(args.first()));
            super::props::write(&current, &h);
            Ok(JsValue::Number(super::read::selected_index(&h) as f64))
        }),
    );
    let current = obj.clone();
    let h = handle.clone();
    obj.borrow_mut().insert(
        "__set:value".into(),
        native("set_select_value", Some(1), move |args| {
            let value = args.first().unwrap_or(&JsValue::Undefined).display();
            super::mutation::set_value(&h, &value);
            super::props::write(&current, &h);
            Ok(JsValue::String(super::read::value(&h)))
        }),
    );
}

fn index_arg(value: Option<&JsValue>) -> isize {
    match value.unwrap_or(&JsValue::Undefined) {
        JsValue::Number(n) if n.is_finite() => n.trunc() as isize,
        other => other.display().parse().unwrap_or(-1),
    }
}
