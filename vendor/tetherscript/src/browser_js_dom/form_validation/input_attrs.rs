use super::*;

pub(super) fn install(
    obj: &Rc<RefCell<HashMap<String, JsValue>>>,
    handle: &DomHandle,
    el: &Element,
) {
    {
        let mut map = obj.borrow_mut();
        map.insert(
            "defaultValue".into(),
            JsValue::String(el.attrs.get("value").cloned().unwrap_or_default()),
        );
        map.insert(
            "defaultChecked".into(),
            JsValue::Bool(el.attrs.contains_key("checked")),
        );
    }
    default_value(obj, handle);
    default_checked(obj, handle);
}

fn default_value(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &DomHandle) {
    let h = handle.clone();
    let current = obj.clone();
    obj.borrow_mut().insert(
        "__set:defaultValue".into(),
        native("set_defaultValue", Some(1), move |args| {
            let value = args.first().unwrap_or(&JsValue::Undefined).display();
            attr_update::set(&h, "value", value.clone())?;
            input_attr_sync::value(&current, &value);
            Ok(JsValue::String(value))
        }),
    );
}

fn default_checked(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &DomHandle) {
    let h = handle.clone();
    let current = obj.clone();
    obj.borrow_mut().insert(
        "__set:defaultChecked".into(),
        native("set_defaultChecked", Some(1), move |args| {
            let checked = args.first().unwrap_or(&JsValue::Undefined).truthy();
            if checked {
                attr_update::set(&h, "checked", String::new())?;
            } else {
                attr_update::remove(&h, "checked")?;
            }
            input_attr_sync::checked(&current, checked);
            Ok(JsValue::Bool(checked))
        }),
    );
}
