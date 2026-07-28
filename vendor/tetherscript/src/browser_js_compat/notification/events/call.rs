use super::*;

pub(super) fn all(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    listeners: &ListenerList,
    kind: &str,
    event: &JsValue,
) -> Result<(), String> {
    call_handler(object, kind, event)?;
    for listener in listener_snapshot(listeners, kind) {
        call_this(listener, object, event)?;
    }
    Ok(())
}

fn call_handler(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    kind: &str,
    event: &JsValue,
) -> Result<(), String> {
    let handler = object.borrow().get(&format!("on{kind}")).cloned();
    if let Some(handler) = handler {
        if !matches!(handler, JsValue::Undefined | JsValue::Null) {
            call_this(handler, object, event)?;
        }
    }
    Ok(())
}

fn listener_snapshot(listeners: &ListenerList, kind: &str) -> Vec<JsValue> {
    listeners
        .borrow()
        .iter()
        .filter(|(name, _)| name == kind)
        .map(|(_, listener)| listener.clone())
        .collect()
}

fn call_this(
    callback: JsValue,
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    event: &JsValue,
) -> Result<(), String> {
    js::call_function_with_this(
        callback,
        JsValue::Object(object.clone()),
        std::slice::from_ref(event),
    )?;
    Ok(())
}
