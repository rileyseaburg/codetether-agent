use super::*;

pub(super) fn uint32(value: &JsValue) -> JsValue {
    JsValue::Number(uint32_number(value))
}

pub(super) fn uint16(value: &JsValue) -> JsValue {
    JsValue::Number(uint32_number(value).rem_euclid(65_536.0))
}

pub(super) fn int32(value: &JsValue) -> JsValue {
    let value = uint32_number(value);
    if value >= 2_147_483_648.0 {
        JsValue::Number(value - 4_294_967_296.0)
    } else {
        JsValue::Number(value)
    }
}

pub(super) fn float32(value: &JsValue) -> JsValue {
    match value {
        JsValue::Number(number) if number.is_finite() => JsValue::Number((*number as f32) as f64),
        JsValue::Bool(true) => JsValue::Number(1.0),
        _ => JsValue::Number(0.0),
    }
}

fn uint32_number(value: &JsValue) -> f64 {
    match value {
        JsValue::Number(number) if number.is_finite() => number.trunc().rem_euclid(4_294_967_296.0),
        JsValue::Bool(true) => 1.0,
        _ => 0.0,
    }
}
