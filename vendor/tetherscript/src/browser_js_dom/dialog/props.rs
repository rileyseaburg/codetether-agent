use super::super::*;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle, el: &Element) {
    obj.insert("open".into(), JsValue::Bool(el.attrs.contains_key("open")));
    obj.insert(
        "returnValue".into(),
        JsValue::String(el.attrs.get("returnvalue").cloned().unwrap_or_default()),
    );
    install_open(obj, handle);
    install_return_value(obj, handle);
    install_handler(obj, handle, "onclose");
    install_handler(obj, handle, "oncancel");
}

fn install_open(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    let h = handle.clone();
    obj.insert(
        "__set:open".into(),
        native("set_dialog_open", Some(1), move |args| {
            if args.first().unwrap_or(&JsValue::Undefined).truthy() {
                attr_update::set(&h, "open", String::new())?;
            } else {
                attr_update::remove(&h, "open")?;
            }
            Ok(JsValue::Bool(attr_update::value(&h, "open").is_some()))
        }),
    );
}

fn install_return_value(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    let h = handle.clone();
    obj.insert(
        "__set:returnValue".into(),
        native("set_dialog_returnValue", Some(1), move |args| {
            let value = args.first().unwrap_or(&JsValue::Undefined).display();
            attr_update::set(&h, "returnvalue", value.clone())?;
            Ok(JsValue::String(value))
        }),
    );
}

fn install_handler(obj: &mut HashMap<String, JsValue>, handle: &DomHandle, prop: &'static str) {
    let h = handle.clone();
    obj.insert(prop.into(), JsValue::Null);
    obj.insert(
        format!("__set:{}", prop),
        native(&format!("set_{}", prop), Some(1), move |args| {
            h.set_handler(prop, args.first().cloned().unwrap_or(JsValue::Undefined));
            Ok(JsValue::Undefined)
        }),
    );
}
