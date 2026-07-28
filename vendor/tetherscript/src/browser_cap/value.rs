//! TetherScript value helpers for browser capability payloads.

use std::cell::RefCell;
use std::rc::Rc;

use crate::value::Value;

pub(crate) fn str_value(s: impl Into<String>) -> Value {
    Value::Str(Rc::new(s.into()))
}

pub(crate) fn map_value(entries: Vec<(&str, Value)>) -> Value {
    let owned = entries.into_iter().map(|(k, v)| (k.to_string(), v));
    Value::Map(Rc::new(RefCell::new(owned.collect())))
}

pub(crate) fn owned_map(entries: Vec<(String, Value)>) -> Value {
    Value::Map(Rc::new(RefCell::new(entries.into_iter().collect())))
}

pub(crate) fn list_str(values: Vec<String>) -> Value {
    Value::List(Rc::new(RefCell::new(
        values.into_iter().map(str_value).collect(),
    )))
}

pub(crate) fn opt_str(value: &Option<String>) -> Value {
    value.clone().map(str_value).unwrap_or(Value::Nil)
}

pub(crate) fn with_action(action: &str, params: &Value) -> Result<Value, String> {
    let mut entries = vec![("action".to_string(), str_value(action))];
    match params {
        Value::Nil => {}
        Value::Map(m) => entries.extend(
            m.borrow()
                .iter()
                .filter(|(k, _)| *k != "action")
                .map(|(k, v)| (k.clone(), v.clone())),
        ),
        other => {
            return Err(format!(
                "browser.raw params must be map, got {}",
                other.type_name()
            ))
        }
    }
    Ok(owned_map(entries))
}
