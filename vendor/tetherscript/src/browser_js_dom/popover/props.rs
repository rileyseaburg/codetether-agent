use super::super::*;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle, el: &Element) {
    obj.insert("popoverOpen".into(), JsValue::Bool(is_open(el)));
    install_setter(obj, handle);
}

fn is_open(el: &Element) -> bool {
    el.attrs.contains_key("popover-open")
}

fn install_setter(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    let h = handle.clone();
    obj.insert(
        "__set:popoverOpen".into(),
        native("set_popoverOpen", Some(1), move |args| {
            let next = args.first().unwrap_or(&JsValue::Undefined).truthy();
            set_state(&h, next)?;
            Ok(JsValue::Bool(next))
        }),
    );
}

pub(super) fn set_state(handle: &DomHandle, open: bool) -> Result<(), String> {
    if open {
        attr_update::set(handle, "popover-open", String::new())
    } else {
        attr_update::remove(handle, "popover-open")
    }
}
