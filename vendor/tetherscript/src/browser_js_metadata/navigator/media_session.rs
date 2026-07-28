use super::*;
use std::cell::RefCell;
use std::rc::Rc;

#[path = "media_session/action.rs"]
mod action;
#[path = "media_session/position.rs"]
mod position;
#[path = "media_session/value.rs"]
mod value;

pub(super) type JsObject = Rc<RefCell<HashMap<String, JsValue>>>;

pub(super) fn install(navigator: &mut HashMap<String, JsValue>) {
    navigator.insert("mediaSession".into(), object());
}

fn object() -> JsValue {
    let session = base();
    session
        .borrow_mut()
        .insert("setActionHandler".into(), action::method(session.clone()));
    session
        .borrow_mut()
        .insert("setPositionState".into(), position::method(session.clone()));
    JsValue::Object(session)
}

fn base() -> JsObject {
    Rc::new(RefCell::new(HashMap::from([
        ("metadata".into(), JsValue::Null),
        ("playbackState".into(), JsValue::String("none".into())),
        ("__actionHandlers".into(), value::empty_object()),
        ("__positionState".into(), JsValue::Undefined),
    ])))
}
