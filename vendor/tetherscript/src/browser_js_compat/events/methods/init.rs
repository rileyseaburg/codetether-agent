use super::*;

pub(super) fn install(
    event: &Rc<RefCell<HashMap<String, JsValue>>>,
    event_class: class::EventClass,
) {
    install_event(event);
    if event_class.has_custom_init() {
        install_custom(event);
    }
}

fn install_event(event: &Rc<RefCell<HashMap<String, JsValue>>>) {
    let object = event.clone();
    event.borrow_mut().insert(
        "initEvent".into(),
        native("Event.initEvent", Some(3), move |args| {
            write_common(&object, args);
            Ok(JsValue::Undefined)
        }),
    );
}

fn install_custom(event: &Rc<RefCell<HashMap<String, JsValue>>>) {
    let object = event.clone();
    event.borrow_mut().insert(
        "initCustomEvent".into(),
        native("CustomEvent.initCustomEvent", Some(4), move |args| {
            write_common(&object, args);
            object.borrow_mut().insert(
                "detail".into(),
                args.get(3).cloned().unwrap_or(JsValue::Null),
            );
            Ok(JsValue::Undefined)
        }),
    );
}

fn write_common(event: &Rc<RefCell<HashMap<String, JsValue>>>, args: &[JsValue]) {
    let mut event = event.borrow_mut();
    let event_type = args.first().unwrap_or(&JsValue::Undefined).display();
    event.insert("type".into(), JsValue::String(event_type));
    event.insert(
        "bubbles".into(),
        JsValue::Bool(args.get(1).is_some_and(JsValue::truthy)),
    );
    event.insert(
        "cancelable".into(),
        JsValue::Bool(args.get(2).is_some_and(JsValue::truthy)),
    );
    event.insert("defaultPrevented".into(), JsValue::Bool(false));
}
