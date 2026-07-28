use crate::value::Value;

use super::url::normalize_origin;

pub(super) fn apply(value: Option<&Value>, origins: &mut Vec<String>) -> Result<(), String> {
    let Some(value) = value else {
        return Ok(());
    };
    let requested = requested_origins(value)?;
    origins.retain(|origin| requested.iter().any(|allowed| allowed == origin));
    if origins.is_empty() {
        return Err("http.narrow: requested origins are not a subset of current scope".into());
    }
    Ok(())
}

fn requested_origins(value: &Value) -> Result<Vec<String>, String> {
    let values = match value {
        Value::List(values) => values.borrow(),
        _ => return Err("http.narrow: `origins` must be a list of strings".into()),
    };
    values
        .iter()
        .map(|value| match value {
            Value::Str(origin) => Ok(normalize_origin((**origin).clone())),
            _ => Err("http.narrow: origins must be strings".to_string()),
        })
        .collect()
}
