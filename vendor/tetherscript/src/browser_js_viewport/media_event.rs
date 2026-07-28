use super::*;

pub(super) fn install(
    map: &mut HashMap<String, JsValue>,
    object: Rc<RefCell<HashMap<String, JsValue>>>,
    listeners: media_object::Listeners,
) {
    let add_listeners = listeners.clone();
    map.insert(
        "addEventListener".into(),
        native("MediaQueryList.addEventListener", None, move |args| {
            if is_change(args.first()) {
                media_legacy::add(&add_listeners, args.get(1).cloned());
            }
            Ok(JsValue::Undefined)
        }),
    );
    let remove_listeners = listeners.clone();
    map.insert(
        "removeEventListener".into(),
        native("MediaQueryList.removeEventListener", None, move |args| {
            if is_change(args.first()) {
                media_legacy::remove(&remove_listeners, args.get(1));
            }
            Ok(JsValue::Undefined)
        }),
    );
    map.insert(
        "dispatchEvent".into(),
        native("MediaQueryList.dispatchEvent", Some(1), move |args| {
            media_dispatch::dispatch(
                object.clone(),
                listeners.clone(),
                args.first().cloned().unwrap_or(JsValue::Undefined),
            )
        }),
    );
}

fn is_change(value: Option<&JsValue>) -> bool {
    value.is_some_and(|value| value.display() == "change")
}
