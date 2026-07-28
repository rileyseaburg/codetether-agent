use super::*;
use crate::value::Value;

#[path = "convert/object.rs"]
mod object;

pub(super) fn value_to_js(value: &Value) -> JsValue {
    match value {
        Value::Nil => JsValue::Null,
        Value::Bool(value) => JsValue::Bool(*value),
        Value::Int(value) => JsValue::Number(*value as f64),
        Value::Float(value) => JsValue::Number(*value),
        Value::Str(value) => JsValue::String((**value).clone()),
        Value::List(values) => JsValue::Array(Rc::new(RefCell::new(
            values.borrow().iter().map(value_to_js).collect(),
        ))),
        Value::Map(values) => JsValue::Object(Rc::new(RefCell::new(
            values
                .borrow()
                .iter()
                .map(|(key, value)| (key.clone(), value_to_js(value)))
                .collect(),
        ))),
        _ => JsValue::Null,
    }
}

pub(super) fn js_to_value(value: &JsValue) -> Value {
    match value {
        JsValue::Undefined | JsValue::Null => Value::Nil,
        JsValue::Bool(value) => Value::Bool(*value),
        JsValue::Number(value) if value.fract() == 0.0 => Value::Int(*value as i64),
        JsValue::Number(value) => Value::Float(*value),
        JsValue::String(value) => Value::Str(Rc::new(value.clone())),
        JsValue::Array(values) => Value::List(Rc::new(RefCell::new(
            values.borrow().iter().map(js_to_value).collect(),
        ))),
        JsValue::Object(values) => {
            Value::Map(Rc::new(RefCell::new(object::to_map(&values.borrow()))))
        }
        _ => Value::Nil,
    }
}
