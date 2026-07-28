use super::*;
use crate::value::Value;

pub(super) fn to_map(values: &HashMap<String, JsValue>) -> HashMap<String, Value> {
    values
        .iter()
        .filter(|(key, value)| !key.starts_with("__") && json_property(value))
        .map(|(key, value)| (key.clone(), super::js_to_value(value)))
        .collect()
}

fn json_property(value: &JsValue) -> bool {
    !matches!(
        value,
        JsValue::Undefined
            | JsValue::Function(_)
            | JsValue::BoundFunction(_)
            | JsValue::Class(_)
            | JsValue::Native(_)
    )
}
