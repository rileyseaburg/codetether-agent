use super::*;

pub(super) fn read(value: Option<&JsValue>) -> f64 {
    match value.unwrap_or(&JsValue::Undefined) {
        JsValue::Number(n) => *n,
        JsValue::String(s) => s.trim().parse().unwrap_or(f64::NAN),
        JsValue::Bool(true) => 1.0,
        JsValue::Bool(false) | JsValue::Null => 0.0,
        _ => f64::NAN,
    }
}

pub(super) fn display(number: f64) -> String {
    if number.fract() == 0.0 {
        format!("{}", number as i64)
    } else {
        number.to_string()
    }
}
