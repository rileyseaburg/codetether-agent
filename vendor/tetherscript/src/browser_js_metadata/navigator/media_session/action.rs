use super::*;

pub(super) fn method(session: JsObject) -> JsValue {
    native(
        "navigator.mediaSession.setActionHandler",
        None,
        move |args| {
            let action = args.first().unwrap_or(&JsValue::Undefined).display();
            let handler = args.get(1).cloned().unwrap_or(JsValue::Undefined);
            let Some(handlers) = handlers(&session) else {
                return Ok(JsValue::Undefined);
            };
            update(handlers, action, handler);
            Ok(JsValue::Undefined)
        },
    )
}

fn handlers(session: &JsObject) -> Option<JsObject> {
    match session.borrow().get("__actionHandlers").cloned() {
        Some(JsValue::Object(handlers)) => Some(handlers),
        _ => None,
    }
}

fn update(handlers: JsObject, action: String, handler: JsValue) {
    if matches!(handler, JsValue::Null | JsValue::Undefined) {
        handlers.borrow_mut().remove(&action);
    } else {
        handlers.borrow_mut().insert(action, handler);
    }
}
