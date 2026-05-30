use tetherscript::value::Value;

/// Read a string list from computer narrowing params.
pub fn requested(params: &Value, key: &str) -> Result<Option<Vec<String>>, String> {
    match params {
        Value::List(items) if key == "scopes" => Ok(Some(strings(items))),
        Value::Map(map) => match map.borrow().get(key) {
            Some(Value::List(items)) => Ok(Some(strings(items))),
            Some(_) => Err(format!("computer.narrow: expected {key} list")),
            None => Ok(None),
        },
        _ => Err("computer.narrow: expected map or list".into()),
    }
}

fn strings(items: &std::cell::RefCell<Vec<Value>>) -> Vec<String> {
    items
        .borrow()
        .iter()
        .filter_map(|v| match v {
            Value::Str(s) => Some(s.trim_end_matches('/').to_string()),
            _ => None,
        })
        .collect()
}
