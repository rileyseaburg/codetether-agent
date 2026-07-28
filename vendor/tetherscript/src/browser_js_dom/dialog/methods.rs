use super::super::*;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    install_show(obj, handle, "show");
    install_show(obj, handle, "showModal");
    install_close(obj, handle);
}

fn install_show(obj: &mut HashMap<String, JsValue>, handle: &DomHandle, name: &'static str) {
    let h = handle.clone();
    obj.insert(
        name.into(),
        native(&format!("HTMLDialogElement.{}", name), Some(0), move |_| {
            attr_update::set(&h, "open", String::new())?;
            Ok(JsValue::Undefined)
        }),
    );
}

fn install_close(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    let h = handle.clone();
    obj.insert(
        "close".into(),
        native("HTMLDialogElement.close", None, move |args| {
            if let Some(value) = args.first() {
                attr_update::set(&h, "returnvalue", value.display())?;
            }
            let was_open = attr_update::value(&h, "open").is_some();
            attr_update::remove(&h, "open")?;
            if was_open {
                h.dispatch_event(JsValue::String("close".into()))?;
            }
            Ok(JsValue::Undefined)
        }),
    );
}
