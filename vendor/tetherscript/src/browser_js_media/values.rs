use super::*;

pub(super) fn number(value: Option<&JsValue>, default: f64) -> f64 {
    match value {
        Some(JsValue::Number(number)) if number.is_finite() => *number,
        Some(value) => value.display().parse::<f64>().unwrap_or(default),
        None => default,
    }
}

pub(super) fn bool_value(value: Option<&JsValue>) -> bool {
    value.unwrap_or(&JsValue::Undefined).truthy()
}
