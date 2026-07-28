use super::*;

pub(super) fn is_array_buffer(object: &HashMap<String, JsValue>) -> bool {
    matches!(object.get("__array_buffer"), Some(JsValue::Bool(true)))
}

pub(super) fn len(object: &HashMap<String, JsValue>) -> usize {
    number::usize(object.get("length"), 0)
}
