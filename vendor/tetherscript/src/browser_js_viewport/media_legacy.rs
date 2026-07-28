use super::*;

pub(super) fn install(map: &mut HashMap<String, JsValue>, listeners: media_object::Listeners) {
    let add_listeners = listeners.clone();
    map.insert(
        "addListener".into(),
        native("MediaQueryList.addListener", Some(1), move |args| {
            add(&add_listeners, args.first().cloned());
            Ok(JsValue::Undefined)
        }),
    );
    map.insert(
        "removeListener".into(),
        native("MediaQueryList.removeListener", Some(1), move |args| {
            remove(&listeners, args.first());
            Ok(JsValue::Undefined)
        }),
    );
}

pub(super) fn add(listeners: &media_object::Listeners, listener: Option<JsValue>) {
    if let Some(listener) = listener {
        listeners.borrow_mut().push(listener);
    }
}

pub(super) fn remove(listeners: &media_object::Listeners, listener: Option<&JsValue>) {
    if let Some(listener) = listener {
        listeners.borrow_mut().retain(|item| item != listener);
    }
}
