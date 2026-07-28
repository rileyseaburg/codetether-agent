//! Convert TetherScript values into HTTP response parts.

use std::collections::HashMap;

use crate::value::Value;

pub(crate) type Parts = (u16, HashMap<String, String>, Vec<u8>);

/// Extract status, headers, and body bytes from a script response value.
pub(crate) fn extract(resp: &Value) -> Result<Parts, String> {
    match resp {
        Value::Str(s) => Ok((200, HashMap::new(), s.as_bytes().to_vec())),
        Value::Nil => Ok((204, HashMap::new(), Vec::new())),
        Value::Map(m) => extract_map(&m.borrow()),
        other => Err(format!(
            "handler must return a str or map, got {}",
            other.type_name()
        )),
    }
}

fn extract_map(m: &HashMap<String, Value>) -> Result<Parts, String> {
    let status = match m.get("status") {
        Some(Value::Int(n)) => *n as u16,
        Some(other) => {
            return Err(format!(
                "response.status must be int, got {}",
                other.type_name()
            ))
        }
        None => 200,
    };
    let body = match m.get("body") {
        Some(Value::Str(s)) => s.as_bytes().to_vec(),
        Some(Value::Nil) | None => Vec::new(),
        Some(other) => other.to_string().into_bytes(),
    };
    Ok((status, extract_headers(m), body))
}

fn extract_headers(m: &HashMap<String, Value>) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    if let Some(Value::Map(h)) = m.get("headers") {
        for (k, v) in h.borrow().iter() {
            headers.insert(k.to_ascii_lowercase(), v.to_string());
        }
    }
    headers
}
