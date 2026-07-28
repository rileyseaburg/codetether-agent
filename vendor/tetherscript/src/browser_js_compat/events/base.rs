use super::*;

pub(super) fn insert_common(
    map: &mut HashMap<String, JsValue>,
    event_type: String,
    init: Option<&JsValue>,
) {
    map.insert("type".into(), JsValue::String(event_type));
    map.insert("bubbles".into(), JsValue::Bool(bool_prop(init, "bubbles")));
    map.insert(
        "cancelable".into(),
        JsValue::Bool(bool_prop(init, "cancelable")),
    );
    map.insert(
        "composed".into(),
        JsValue::Bool(bool_prop(init, "composed")),
    );
    map.insert("defaultPrevented".into(), JsValue::Bool(false));
    map.insert("timeStamp".into(), JsValue::Number(0.0));
    map.insert("target".into(), JsValue::Null);
    map.insert("currentTarget".into(), JsValue::Null);
    map.insert("eventPhase".into(), JsValue::Number(0.0));
}

pub(super) fn prop(init: Option<&JsValue>, name: &str) -> Option<JsValue> {
    match init {
        Some(JsValue::Object(obj)) => obj.borrow().get(name).cloned(),
        _ => None,
    }
}

pub(super) fn bool_prop(init: Option<&JsValue>, name: &str) -> bool {
    prop(init, name).is_some_and(|value| value.truthy())
}

pub(super) fn num_prop(init: Option<&JsValue>, name: &str, default: f64) -> f64 {
    prop(init, name)
        .map(|value| number_value(&value))
        .unwrap_or(default)
}

pub(super) fn string_prop(init: Option<&JsValue>, name: &str) -> String {
    prop(init, name)
        .map(|value| value.display())
        .unwrap_or_default()
}

fn number_value(value: &JsValue) -> f64 {
    match value {
        JsValue::Number(n) => *n,
        JsValue::Bool(true) => 1.0,
        JsValue::Bool(false) | JsValue::Null => 0.0,
        JsValue::String(s) => s.parse().unwrap_or(f64::NAN),
        _ => f64::NAN,
    }
}
