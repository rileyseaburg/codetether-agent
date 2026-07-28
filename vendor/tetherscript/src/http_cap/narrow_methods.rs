use std::collections::HashSet;

use crate::value::Value;

pub(super) fn apply(
    value: Option<&Value>,
    methods: &mut HashSet<String>,
) -> Result<(), String> {
    let Some(value) = value else {
        return Ok(());
    };
    let requested = requested_methods(value)?;
    *methods = methods.intersection(&requested).cloned().collect();
    if methods.is_empty() {
        return Err("http.narrow: no methods left after intersection".into());
    }
    Ok(())
}

fn requested_methods(value: &Value) -> Result<HashSet<String>, String> {
    let values = match value {
        Value::List(values) => values.borrow(),
        _ => return Err("http.narrow: `methods` must be a list of strings".into()),
    };
    values
        .iter()
        .map(|value| match value {
            Value::Str(method) => Ok(method.to_ascii_uppercase()),
            _ => Err("http.narrow: methods must be strings".to_string()),
        })
        .collect()
}
