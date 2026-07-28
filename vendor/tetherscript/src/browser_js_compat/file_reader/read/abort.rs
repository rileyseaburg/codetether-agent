use super::*;

pub(super) fn install(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    listeners: events::ListenerList,
) {
    let reader = object.clone();
    object.borrow_mut().insert(
        "abort".into(),
        native("FileReader.abort", Some(0), move |_| {
            abort(&reader, &listeners)
        }),
    );
}

fn abort(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    listeners: &events::ListenerList,
) -> Result<JsValue, String> {
    if !state::is_loading(object) {
        return Ok(JsValue::Undefined);
    }
    state::set(object, "readyState", JsValue::Number(2.0));
    state::set(object, "result", JsValue::Null);
    state::set(object, "error", JsValue::String("AbortError".into()));
    events::dispatch(object, listeners, "abort")?;
    events::dispatch(object, listeners, "loadend")?;
    Ok(JsValue::Undefined)
}
