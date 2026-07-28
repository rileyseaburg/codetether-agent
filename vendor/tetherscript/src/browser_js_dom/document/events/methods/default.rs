use super::*;

pub(super) fn install(event: &Rc<RefCell<HashMap<String, JsValue>>>) {
    let event_for_prevent = event.clone();
    event.borrow_mut().insert(
        "preventDefault".into(),
        native("Event.preventDefault", Some(0), move |_| {
            if event_for_prevent
                .borrow()
                .get("cancelable")
                .is_some_and(JsValue::truthy)
            {
                event_for_prevent
                    .borrow_mut()
                    .insert("defaultPrevented".into(), JsValue::Bool(true));
            }
            Ok(JsValue::Undefined)
        }),
    );
}
