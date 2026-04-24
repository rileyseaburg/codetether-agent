use std::cell::RefCell;
use std::rc::Rc;

use kiln::value::Value as KilnValue;
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
        Value::Object(values) => object_to_kiln(values),
    }
}

fn object_to_kiln(values: Map<String, Value>) -> KilnValue {
    KilnValue::Map(Rc::new(RefCell::new(
        values
            .into_iter()
            .map(|(key, value)| (key, json_to_kiln(value)))
            .collect(),
    )))
}

fn number_to_kiln(number: Number) -> KilnValue {
    if let Some(value) = number.as_i64() {
        KilnValue::Int(value)
    } else if let Some(value) = number.as_u64() {
        match i64::try_from(value) {
            Ok(value) => KilnValue::Int(value),
            Err(_) => KilnValue::Str(Rc::new(value.to_string())),
        }
    } else if let Some(value) = number.as_f64() {
        KilnValue::Float(value)
    } else {
        KilnValue::Str(Rc::new(number.to_string()))
    }
}
