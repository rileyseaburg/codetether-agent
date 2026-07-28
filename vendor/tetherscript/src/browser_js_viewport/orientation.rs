use super::super::*;

#[path = "orientation_event.rs"]
mod event;
#[path = "orientation_events.rs"]
mod events;
#[path = "orientation_lock.rs"]
mod lock;
#[cfg(test)]
#[path = "tests_orientation_events.rs"]
mod tests_orientation_events;

pub(super) fn object() -> JsValue {
    let object = Rc::new(RefCell::new(HashMap::from([
        ("type".into(), JsValue::String("landscape-primary".into())),
        ("angle".into(), JsValue::Number(0.0)),
        ("onchange".into(), JsValue::Null),
    ])));
    events::install(&object);
    lock::install(&object);
    JsValue::Object(object)
}
