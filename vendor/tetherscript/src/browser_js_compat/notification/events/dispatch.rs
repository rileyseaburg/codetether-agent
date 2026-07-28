use super::*;

pub(super) fn install_methods(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    listeners: ListenerList,
) {
    let dispatch_object = object.clone();
    let dispatch_listeners = listeners.clone();
    object.borrow_mut().insert(
        "dispatchEvent".into(),
        native("Notification.dispatchEvent", Some(1), move |args| {
            let event = event::from_arg(args.first());
            dispatch(&dispatch_object, &dispatch_listeners, event)
        }),
    );
    let close_object = object.clone();
    object.borrow_mut().insert(
        "close".into(),
        native("Notification.close", Some(0), move |_| {
            close_object
                .borrow_mut()
                .insert("closed".into(), JsValue::Bool(true));
            dispatch_kind(&close_object, &listeners, "close")
        }),
    );
}

fn dispatch_kind(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    listeners: &ListenerList,
    kind: &str,
) -> Result<JsValue, String> {
    dispatch(object, listeners, event::fresh(kind))
}

fn dispatch(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    listeners: &ListenerList,
    event: JsValue,
) -> Result<JsValue, String> {
    let kind = event::kind(&event);
    event::prepare(&event, object, &kind);
    call::all(object, listeners, &kind, &event)?;
    Ok(JsValue::Bool(true))
}
