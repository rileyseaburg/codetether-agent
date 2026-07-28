use super::*;

#[derive(Clone)]
pub(super) enum PromiseState {
    Pending,
    Fulfilled(JsValue),
    Rejected(JsValue),
}

pub(super) fn settle(value: JsValue) -> PromiseState {
    from_value(&value).unwrap_or(PromiseState::Fulfilled(value))
}

pub(super) fn from_value(value: &JsValue) -> Option<PromiseState> {
    let JsValue::Object(object) = value else {
        return None;
    };
    let object = object.borrow();
    let JsValue::String(state) = object.get("__promise_state")? else {
        return None;
    };
    match state.as_str() {
        "fulfilled" => Some(PromiseState::Fulfilled(first(
            &object,
            &["__promise_value", "value"],
        ))),
        "rejected" => Some(PromiseState::Rejected(first(
            &object,
            &["__promise_reason", "reason"],
        ))),
        "pending" => Some(PromiseState::Pending),
        _ => None,
    }
}

fn first(object: &HashMap<String, JsValue>, names: &[&str]) -> JsValue {
    names
        .iter()
        .find_map(|name| object.get(*name).cloned())
        .unwrap_or(JsValue::Undefined)
}
