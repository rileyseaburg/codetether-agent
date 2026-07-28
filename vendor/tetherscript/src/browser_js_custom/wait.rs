use super::*;

pub(super) fn thenable(name: String, ready: Option<JsValue>) -> JsValue {
    let state = Rc::new(RefCell::new(ready));
    let mut obj = HashMap::new();
    let state_for_then = state.clone();
    obj.insert(
        "then".into(),
        native("customElements.whenDefined.then", Some(1), move |args| {
            let callback = args.first().cloned().unwrap_or(JsValue::Undefined);
            if !util::callable(&callback) {
                return Ok(JsValue::Undefined);
            }
            if let Some(value) = state_for_then
                .borrow()
                .clone()
                .or_else(|| registry::get(&name).map(|definition| definition.value))
            {
                js::call_function_with_this(callback, JsValue::Undefined, &[value])?;
            } else {
                registry::wait(name.clone(), callback);
            }
            Ok(JsValue::Undefined)
        }),
    );
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

pub(super) fn notify(name: &str, definition: JsValue) -> Result<(), String> {
    for callback in registry::take_waiters(name) {
        js::call_function_with_this(
            callback,
            JsValue::Undefined,
            std::slice::from_ref(&definition),
        )?;
    }
    Ok(())
}
