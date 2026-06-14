use tetherscript::value::Value;

pub fn string_arg(value: &Value, label: &str) -> Result<String, String> {
    match value {
        Value::Str(value) => Ok((**value).clone()),
        other => Err(format!("{label}: expected str, got {}", other.type_name())),
    }
}

pub fn list_arg(value: &Value, label: &str) -> Result<Vec<String>, String> {
    match value {
        Value::Nil => Ok(Vec::new()),
        Value::List(values) => values
            .borrow()
            .iter()
            .map(|value| string_arg(value, label))
            .collect(),
        other => Err(format!("{label}: expected list, got {}", other.type_name())),
    }
}

pub fn timeout_arg(value: &Value) -> Result<u64, String> {
    match value {
        Value::Nil => Ok(30_000),
        Value::Int(value) if *value > 0 => Ok(*value as u64),
        _ => Err("process_run: timeout_ms must be a positive integer".into()),
    }
}
