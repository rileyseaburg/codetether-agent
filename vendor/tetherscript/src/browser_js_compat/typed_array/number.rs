use super::*;

pub(super) fn usize(value: Option<&JsValue>, default: usize) -> usize {
    match value {
        Some(JsValue::Number(number)) if number.is_finite() && *number > 0.0 => *number as usize,
        _ => default,
    }
}

pub(super) fn signed(value: &JsValue) -> f64 {
    match value {
        JsValue::Number(number) if number.is_finite() => *number,
        JsValue::Bool(true) => 1.0,
        JsValue::String(text) => text.parse().unwrap_or(0.0),
        _ => 0.0,
    }
}

pub(super) fn byte(value: Option<&JsValue>) -> u8 {
    match value {
        Some(JsValue::Number(number)) if number.is_finite() => (*number as i64 & 0xff) as u8,
        Some(JsValue::Bool(true)) => 1,
        _ => 0,
    }
}
