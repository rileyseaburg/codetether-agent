use super::*;

pub(super) fn install(event: &Rc<RefCell<HashMap<String, JsValue>>>) {
    let event_for_init = event.clone();
    event.borrow_mut().insert(
        "initEvent".into(),
        native("Event.initEvent", Some(3), move |args| {
            let event_type = args.first().unwrap_or(&JsValue::Undefined).display();
            let bubbles = args.get(1).is_some_and(JsValue::truthy);
            let cancelable = args.get(2).is_some_and(JsValue::truthy);
            let mut event = event_for_init.borrow_mut();
            event.insert("type".into(), JsValue::String(event_type));
            event.insert("bubbles".into(), JsValue::Bool(bubbles));
            event.insert("cancelable".into(), JsValue::Bool(cancelable));
            event.insert("defaultPrevented".into(), JsValue::Bool(false));
            Ok(JsValue::Undefined)
        }),
    );
}
