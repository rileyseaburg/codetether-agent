use super::JsValue;

#[path = "js_await/hook.rs"]
mod hook;
#[path = "js_await/pending.rs"]
mod pending;

pub(crate) use hook::with_drain;

pub(super) fn value(value: JsValue) -> Result<JsValue, String> {
    match state(&value).as_deref() {
        Some("fulfilled") => Ok(field(&value, &["__promise_value", "value"])),
        Some("rejected") => Err(field(&value, &["__promise_reason", "reason"]).display()),
        Some("pending") => pending::wait(value),
        _ => Ok(value),
    }
}

pub(super) fn state(value: &JsValue) -> Option<String> {
    let JsValue::Object(object) = value else {
        return None;
    };
    match object.borrow().get("__promise_state")? {
        JsValue::String(state) => Some(state.clone()),
        _ => None,
    }
}

pub(super) fn field(value: &JsValue, names: &[&str]) -> JsValue {
    let JsValue::Object(object) = value else {
        return JsValue::Undefined;
    };
    let object = object.borrow();
    names
        .iter()
        .find_map(|name| object.get(*name).cloned())
        .unwrap_or(JsValue::Undefined)
}

pub(super) fn drain_once() -> Result<bool, String> {
    hook::drain_once()
}
