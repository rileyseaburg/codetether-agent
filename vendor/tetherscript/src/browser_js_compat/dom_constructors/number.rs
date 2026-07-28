use super::*;

pub(super) fn arg(args: &[JsValue], index: usize, default: f64) -> f64 {
    value(args.get(index), default)
}

pub(super) fn field(source: &JsValue, key: &str, default: f64) -> f64 {
    let Some(obj) = object(source) else {
        return default;
    };
    let stored = obj.borrow().get(key).cloned();
    value(stored.as_ref(), default)
}

pub(super) fn props(fields: &[(&'static str, f64)]) -> HashMap<String, JsValue> {
    fields
        .iter()
        .map(|(key, value)| ((*key).into(), JsValue::Number(*value)))
        .collect()
}

fn value(value: Option<&JsValue>, default: f64) -> f64 {
    match value.unwrap_or(&JsValue::Undefined) {
        JsValue::Undefined => default,
        JsValue::Null => 0.0,
        JsValue::Bool(value) => {
            if *value {
                1.0
            } else {
                0.0
            }
        }
        JsValue::Number(value) => *value,
        JsValue::String(value) => value.trim().parse().unwrap_or(f64::NAN),
        _ => f64::NAN,
    }
}
