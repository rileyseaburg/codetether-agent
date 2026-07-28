use super::*;

pub(super) fn install(object: &Rc<RefCell<HashMap<String, JsValue>>>, listeners: ListenerList) {
    let add_listeners = listeners.clone();
    object.borrow_mut().insert(
        "addEventListener".into(),
        native("Notification.addEventListener", Some(2), move |args| {
            add_listeners.borrow_mut().push((
                args.first().map(JsValue::display).unwrap_or_default(),
                args.get(1).cloned().unwrap_or(JsValue::Undefined),
            ));
            Ok(JsValue::Undefined)
        }),
    );
    object.borrow_mut().insert(
        "removeEventListener".into(),
        native("Notification.removeEventListener", Some(2), move |args| {
            let kind = args.first().map(JsValue::display).unwrap_or_default();
            let listener = args.get(1).cloned().unwrap_or(JsValue::Undefined);
            listeners
                .borrow_mut()
                .retain(|(name, item)| name != &kind || item != &listener);
            Ok(JsValue::Undefined)
        }),
    );
}
