//! Argument validation for `http_serve_static`.

use crate::value::Value;

/// Parse and validate the TCP port argument.
pub(crate) fn port_arg(value: &Value) -> Result<u16, String> {
    match value {
        Value::Int(n) if (1..=65535).contains(n) => Ok(*n as u16),
        Value::Int(n) => Err(format!("http_serve_static: port {} out of range", n)),
        other => Err(format!(
            "http_serve_static: port must be int, got {}",
            other.type_name()
        )),
    }
}

/// Parse and validate the static root directory argument.
pub(crate) fn string_arg(value: &Value) -> Result<String, String> {
    match value {
        Value::Str(value) if !value.is_empty() => Ok((**value).clone()),
        Value::Str(_) => Err("http_serve_static: root_dir must not be empty".into()),
        other => Err(format!(
            "http_serve_static: root_dir must be str, got {}",
            other.type_name()
        )),
    }
}
