//! Browserctl response normalization.

use crate::{json, value::Value};

pub(crate) fn normalize(method: &str, parsed: Value) -> Result<Value, String> {
    if let Value::Map(m) = &parsed {
        let map = m.borrow();
        if let Some(Value::Bool(ok)) = map.get("ok") {
            return bridge_ok(method, *ok, &map);
        }
        if let Some(Value::Bool(success)) = map.get("success") {
            return tool_success(method, *success, &map);
        }
    }
    Ok(parsed)
}

fn bridge_ok(
    method: &str,
    ok: bool,
    map: &std::collections::HashMap<String, Value>,
) -> Result<Value, String> {
    if ok {
        return Ok(map
            .get("value")
            .cloned()
            .or_else(|| map.get("result").cloned())
            .unwrap_or(Value::Nil));
    }
    Err(format!("browser.{}: {}", method, message(map)))
}

fn tool_success(
    method: &str,
    success: bool,
    map: &std::collections::HashMap<String, Value>,
) -> Result<Value, String> {
    if !success {
        return Err(format!("browser.{}: {}", method, message(map)));
    }
    match map.get("output") {
        Some(Value::Str(s)) => Ok(parse_output(s).unwrap_or_else(|| Value::Str(s.clone()))),
        Some(v) => Ok(v.clone()),
        None => Ok(map.get("metadata").cloned().unwrap_or(Value::Nil)),
    }
}

fn parse_output(output: &str) -> Option<Value> {
    json::parse_str(output).ok()
}

fn message(map: &std::collections::HashMap<String, Value>) -> String {
    ["error", "message", "output"]
        .iter()
        .find_map(|key| map.get(*key).map(|v| v.to_string()))
        .unwrap_or_else(|| "unknown bridge error".into())
}
