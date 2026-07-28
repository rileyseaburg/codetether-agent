use super::*;

pub(super) fn method(session: JsObject) -> JsValue {
    native(
        "navigator.mediaSession.setPositionState",
        None,
        move |args| {
            let state = args
                .first()
                .map(value::snapshot)
                .unwrap_or(JsValue::Undefined);
            session.borrow_mut().insert("__positionState".into(), state);
            Ok(JsValue::Undefined)
        },
    )
}
