use super::super::*;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle, el: &Element) {
    obj.entry("disabled".into())
        .or_insert_with(|| JsValue::Bool(el.attrs.contains_key("disabled")));
    if obj.contains_key("__set:disabled") {
        return;
    }
    let h = handle_ref::new(obj, handle);
    obj.insert(
        "__set:disabled".into(),
        native("set_disabled", Some(1), move |args| {
            let next = args.first().unwrap_or(&JsValue::Undefined).truthy();
            if next {
                attr_update::set(&h.current(), "disabled", String::new())?;
            } else {
                attr_update::remove(&h.current(), "disabled")?;
            }
            Ok(JsValue::Bool(next))
        }),
    );
}
