use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use kiln::value::{ResultValue, Value as KilnValue};
use serde_json::{Map, Number, Value};

pub fn json_to_kiln(value: Value) -> KilnValue {
    match value {
        Value::Null => KilnValue::Nil,
        Value::Bool(value) => KilnValue::Bool(value),
        Value::Number(number) => number_to_kiln(number),
        Value::String(value) => KilnValue::Str(Rc::new(value)),
        Value::Array(values) => KilnValue::List(Rc::new(RefCell::new(
            values.into_iter().map(json_to_kiln).collect(),
        ))),
        Value::Object(values) => KilnValue::Map(Rc::new(RefCell::new(
            values
                .into_iter()
                .map(|(key, value)| (key, json_to_kiln(value)))
                .collect(),
        ))),
    }
}

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

fn number_to_kiln(number: Number) -> KilnValue {
    if let Some(value) = number.as_i64() {
        KilnValue::Int(value)
    } else if let Some(value) = number.as_u64().and_then(|value| i64::try_from(value).ok()) {
        KilnValue::Int(value)
    } else {
        KilnValue::Float(number.as_f64().unwrap_or_default())
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
