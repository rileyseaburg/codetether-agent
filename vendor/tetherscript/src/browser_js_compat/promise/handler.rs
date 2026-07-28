use super::*;

pub(super) fn present(value: &JsValue) -> bool {
    !matches!(value, JsValue::Undefined | JsValue::Null)
}
