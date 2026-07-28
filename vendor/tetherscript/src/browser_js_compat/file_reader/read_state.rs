use super::*;

pub(super) fn start(object: &Rc<RefCell<HashMap<String, JsValue>>>) {
    set(object, "readyState", JsValue::Number(1.0));
    set(object, "result", JsValue::Null);
    set(object, "error", JsValue::Null);
}

pub(super) fn finish(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    listeners: &events::ListenerList,
    result: JsValue,
) -> Result<JsValue, String> {
    set(object, "result", result);
    set(object, "readyState", JsValue::Number(2.0));
    events::dispatch(object, listeners, "load")?;
    events::dispatch(object, listeners, "loadend")?;
    Ok(JsValue::Undefined)
}

pub(super) fn fail(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    listeners: &events::ListenerList,
    message: &str,
) -> Result<JsValue, String> {
    set(object, "result", JsValue::Null);
    set(object, "error", JsValue::String(message.into()));
    set(object, "readyState", JsValue::Number(2.0));
    events::dispatch(object, listeners, "error")?;
    events::dispatch(object, listeners, "loadend")?;
    Ok(JsValue::Undefined)
}

pub(super) fn is_loading(object: &Rc<RefCell<HashMap<String, JsValue>>>) -> bool {
    let ready_state = object.borrow().get("readyState").cloned();
    matches!(
        ready_state,
        Some(JsValue::Number(value)) if (value - 1.0).abs() < f64::EPSILON
    )
}

pub(super) fn set(object: &Rc<RefCell<HashMap<String, JsValue>>>, name: &str, value: JsValue) {
    object.borrow_mut().insert(name.into(), value);
}
