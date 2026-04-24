use std::cell::RefCell;
use std::rc::Rc;

use serde_json::{Map, Number, Value};
use tetherscript::value::Value as TetherScriptValue;

pub fn json_to_tetherscript(value: Value) -> TetherScriptValue {
    match value {
        Value::Null => TetherScriptValue::Nil,
        Value::Bool(value) => TetherScriptValue::Bool(value),
        Value::Number(number) => number_to_tetherscript(number),
        Value::String(value) => TetherScriptValue::Str(Rc::new(value)),
        Value::Array(values) => TetherScriptValue::List(Rc::new(RefCell::new(
            values.into_iter().map(json_to_tetherscript).collect(),
        ))),
        Value::Object(values) => object_to_tetherscript(values),
    }
}

fn object_to_tetherscript(values: Map<String, Value>) -> TetherScriptValue {
    TetherScriptValue::Map(Rc::new(RefCell::new(
        values
            .into_iter()
            .map(|(key, value)| (key, json_to_tetherscript(value)))
            .collect(),
    )))
}

fn number_to_tetherscript(number: Number) -> TetherScriptValue {
    if let Some(value) = number.as_i64() {
        TetherScriptValue::Int(value)
    } else if let Some(value) = number.as_u64() {
        match i64::try_from(value) {
            Ok(value) => TetherScriptValue::Int(value),
            Err(_) => TetherScriptValue::Str(Rc::new(value.to_string())),
        }
    } else if let Some(value) = number.as_f64() {
        TetherScriptValue::Float(value)
    } else {
        TetherScriptValue::Str(Rc::new(number.to_string()))
    }
}
