use super::*;

pub(super) fn dispatch(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    listeners: &ListenerList,
    kind: &str,
) -> Result<(), String> {
    let event = event_object(object, kind);
    let handler = object.borrow().get(&format!("on{kind}")).cloned();
    if let Some(handler) = handler {
        if !matches!(handler, JsValue::Undefined) {
            js::call_function_with_this(
                handler,
                JsValue::Object(object.clone()),
                std::slice::from_ref(&event),
            )?;
        }
    }
    for listener in listener_snapshot(listeners, kind) {
        js::call_function_with_this(
            listener,
            JsValue::Object(object.clone()),
            std::slice::from_ref(&event),
        )?;
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

fn event_object(object: &Rc<RefCell<HashMap<String, JsValue>>>, kind: &str) -> JsValue {
    JsValue::Object(Rc::new(RefCell::new(HashMap::from([
        ("type".into(), JsValue::String(kind.into())),
        ("target".into(), JsValue::Object(object.clone())),
    ]))))
}
