use std::collections::HashMap;

use kiln::value::{ResultValue, Value as KilnValue};
use serde_json::{Map, Number, Value};

pub fn kiln_to_json(value: &KilnValue) -> Value {
    match value {
        KilnValue::Nil => Value::Null,
        KilnValue::Int(value) => Value::Number(Number::from(*value)),
        KilnValue::Float(value) => Number::from_f64(*value).map_or(Value::Null, Value::Number),
        KilnValue::Bool(value) => Value::Bool(*value),
        KilnValue::Str(value) => Value::String((**value).clone()),
        KilnValue::List(values) => Value::Array(values.borrow().iter().map(kiln_to_json).collect()),
        KilnValue::Map(values) => Value::Object(map_to_json(&values.borrow())),
        KilnValue::Result(result) => result_to_json(result),
        KilnValue::Fn(_) | KilnValue::VmFn(_) | KilnValue::Native(_) | KilnValue::Capability(_) => {
            Value::String(value.to_string())
        }
    }
}

fn map_to_json(values: &HashMap<String, KilnValue>) -> Map<String, Value> {
    values
        .iter()
        .map(|(key, value)| (key.clone(), kiln_to_json(value)))
        .collect()
}

fn result_to_json(result: &ResultValue) -> Value {
    match result {
        ResultValue::Ok(value) => serde_json::json!({ "ok": kiln_to_json(value) }),
        ResultValue::Err(message) => serde_json::json!({ "err": message }),
    }
}
