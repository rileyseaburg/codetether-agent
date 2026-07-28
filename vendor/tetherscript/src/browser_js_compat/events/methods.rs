use super::*;

#[path = "methods/init.rs"]
mod init;

pub(super) fn install(
    event: &Rc<RefCell<HashMap<String, JsValue>>>,
    event_class: class::EventClass,
) {
    init::install(event, event_class);
    let for_prevent = event.clone();
    event.borrow_mut().insert(
        "preventDefault".into(),
        native("Event.preventDefault", Some(0), move |_| {
            if for_prevent
                .borrow()
                .get("cancelable")
                .is_some_and(JsValue::truthy)
            {
                for_prevent
                    .borrow_mut()
                    .insert("defaultPrevented".into(), JsValue::Bool(true));
            }
            Ok(JsValue::Undefined)
        }),
    );
    let for_stop = event.clone();
    event.borrow_mut().insert(
        "stopPropagation".into(),
        native("Event.stopPropagation", Some(0), move |_| {
            for_stop
                .borrow_mut()
                .insert("__propagationStopped".into(), JsValue::Bool(true));
            Ok(JsValue::Undefined)
        }),
    );
    let for_immediate = event.clone();
    event.borrow_mut().insert(
        "stopImmediatePropagation".into(),
        native("Event.stopImmediatePropagation", Some(0), move |_| {
            let mut event = for_immediate.borrow_mut();
            event.insert("__propagationStopped".into(), JsValue::Bool(true));
            event.insert("__immediatePropagationStopped".into(), JsValue::Bool(true));
            Ok(JsValue::Undefined)
        }),
    );
    event.borrow_mut().insert(
        "composedPath".into(),
        native("Event.composedPath", Some(0), move |_| {
            Ok(JsValue::Array(Rc::new(RefCell::new(Vec::new()))))
        }),
    );
}
