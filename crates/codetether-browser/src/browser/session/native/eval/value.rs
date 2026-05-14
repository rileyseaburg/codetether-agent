use serde_json::{Value, json};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use tetherscript::js::JsValue;

pub(super) fn to_json(value: &JsValue) -> Value {
    match value {
        JsValue::Undefined | JsValue::Null => Value::Null,
        JsValue::Bool(value) => json!(value),
        JsValue::Number(value) if value.is_finite() => json!(value),
        JsValue::Number(_) => Value::Null,
        JsValue::String(value) => json!(value),
        JsValue::Array(items) => array(items),
        JsValue::Object(map) => object(map),
        _ => json!(value.display()),
    }
}

fn array(items: &Rc<RefCell<Vec<JsValue>>>) -> Value {
    Value::Array(items.borrow().iter().map(to_json).collect())
}

fn object(map: &Rc<RefCell<HashMap<String, JsValue>>>) -> Value {
    Value::Object(
        map.borrow()
            .iter()
            .map(|(key, value)| (key.clone(), to_json(value)))
            .collect(),
    )
}
