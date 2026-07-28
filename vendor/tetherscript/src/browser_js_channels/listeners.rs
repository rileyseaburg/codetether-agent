use super::*;

pub(super) fn install(object: &JsObject, label: &str) {
    let add_object = object.clone();
    object.borrow_mut().insert(
        "addEventListener".into(),
        native(&format!("{label}.addEventListener"), Some(2), move |args| {
            let event_type = args.first().unwrap_or(&JsValue::Undefined).display();
            let listener = args.get(1).cloned().unwrap_or(JsValue::Undefined);
            listener_store::push(&add_object, &event_type, listener);
            Ok(JsValue::Undefined)
        }),
    );
    let remove_object = object.clone();
    object.borrow_mut().insert(
        "removeEventListener".into(),
        native(
            &format!("{label}.removeEventListener"),
            Some(2),
            move |args| {
                let event_type = args.first().unwrap_or(&JsValue::Undefined).display();
                let listener = args.get(1).cloned().unwrap_or(JsValue::Undefined);
                listener_store::remove(&remove_object, &event_type, &listener);
                Ok(JsValue::Undefined)
            },
        ),
    );
}

pub(super) fn dispatch(object: &JsObject, event_type: &str, event: JsValue) -> Result<(), String> {
    let this_value = JsValue::Object(object.clone());
    let handler = object
        .borrow()
        .get(&format!("on{event_type}"))
        .cloned()
        .unwrap_or(JsValue::Null);
    if handler.truthy() {
        js::call_function_with_this(handler, this_value.clone(), std::slice::from_ref(&event))?;
    }
    for listener in listener_store::get(object, event_type) {
        js::call_function_with_this(listener, this_value.clone(), std::slice::from_ref(&event))?;
    }
    Ok(())
}
