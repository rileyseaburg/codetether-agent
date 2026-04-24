use std::collections::HashMap;

use serde_json::{Map, Number, Value};
use tetherscript::value::{ResultValue, Value as TetherScriptValue};

pub fn tetherscript_to_json(value: &TetherScriptValue) -> Value {
    match value {
        TetherScriptValue::Nil => Value::Null,
        TetherScriptValue::Int(value) => Value::Number(Number::from(*value)),
        TetherScriptValue::Float(value) => {
            Number::from_f64(*value).map_or(Value::Null, Value::Number)
        }
        TetherScriptValue::Bool(value) => Value::Bool(*value),
        TetherScriptValue::Str(value) => Value::String((**value).clone()),
        TetherScriptValue::List(values) => {
            Value::Array(values.borrow().iter().map(tetherscript_to_json).collect())
        }
        TetherScriptValue::Map(values) => Value::Object(map_to_json(&values.borrow())),
        TetherScriptValue::Result(result) => result_to_json(result),
        TetherScriptValue::Fn(_)
        | TetherScriptValue::VmFn(_)
        | TetherScriptValue::Native(_)
        | TetherScriptValue::Capability(_) => Value::String(value.to_string()),
    }
}

fn map_to_json(values: &HashMap<String, TetherScriptValue>) -> Map<String, Value> {
    values
        .iter()
        .map(|(key, value)| (key.clone(), tetherscript_to_json(value)))
        .collect()
}

fn result_to_json(result: &ResultValue) -> Value {
    match result {
        ResultValue::Ok(value) => serde_json::json!({ "ok": tetherscript_to_json(value) }),
        ResultValue::Err(message) => serde_json::json!({ "err": message }),
    }
}
