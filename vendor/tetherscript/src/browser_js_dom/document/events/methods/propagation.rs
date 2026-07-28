use super::*;

pub(super) fn install(event: &Rc<RefCell<HashMap<String, JsValue>>>) {
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
}
