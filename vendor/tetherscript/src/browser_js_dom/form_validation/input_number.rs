use super::*;

pub(super) fn install(
    obj: &Rc<RefCell<HashMap<String, JsValue>>>,
    handle: &DomHandle,
    el: &Element,
) {
    let mut obj_mut = obj.borrow_mut();
    obj_mut.insert("valueAsNumber".into(), JsValue::Number(read(handle, el)));
    let current = obj.clone();
    let h = handle.clone();
    obj_mut.insert(
        "__set:valueAsNumber".into(),
        native("set_valueAsNumber", Some(1), move |args| {
            let number = input_number_arg::read(args.first());
            let value = if number.is_nan() {
                String::new()
            } else {
                input_number_arg::display(number)
            };
            h.set_input_value(value);
            refresh(&current, &h);
            Ok(JsValue::Number(number))
        }),
    );
}

pub(super) fn refresh(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &DomHandle) {
    let Some(Node::Element(el)) = handle.node() else {
        return;
    };
    let mut obj = obj.borrow_mut();
    obj.insert("value".into(), JsValue::String(handle.input_value()));
    obj.insert("valueAsNumber".into(), JsValue::Number(read(handle, &el)));
}

pub(super) fn read(handle: &DomHandle, el: &Element) -> f64 {
    if !numeric(el) {
        return f64::NAN;
    }
    handle.input_value().trim().parse().unwrap_or(f64::NAN)
}

pub(super) fn numeric(el: &Element) -> bool {
    matches!(controls::input_type(el).as_str(), "number" | "range")
}
