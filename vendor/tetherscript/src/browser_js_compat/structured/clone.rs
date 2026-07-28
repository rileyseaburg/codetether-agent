use crate::js::JsValue;

use super::state::CloneState;

pub(super) fn clone_value(value: &JsValue, state: &mut CloneState) -> Result<JsValue, String> {
    match value {
        JsValue::Undefined => Ok(JsValue::Undefined),
        JsValue::Null => Ok(JsValue::Null),
        JsValue::Bool(value) => Ok(JsValue::Bool(*value)),
        JsValue::Number(value) => Ok(JsValue::Number(*value)),
        JsValue::String(value) => Ok(JsValue::String(value.clone())),
        JsValue::Symbol(_) => Err("structuredClone: cannot clone symbol value".into()),
        JsValue::Array(items) => super::array::clone(items, state),
        JsValue::Object(object) => super::object::clone(object, state),
        JsValue::Function(_) | JsValue::BoundFunction(_) | JsValue::Class(_) => {
            Err("structuredClone: cannot clone function value".into())
        }
        JsValue::Native(_) => Err("structuredClone: cannot clone native function value".into()),
    }
}
