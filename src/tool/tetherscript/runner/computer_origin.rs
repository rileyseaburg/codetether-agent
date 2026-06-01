use tetherscript::value::Value;

use super::computer::ComputerAuthority;

/// Ensure a computer action payload is allowed by the authority origins.
pub fn require(auth: &ComputerAuthority, method: &str, payload: &Value) -> Result<(), String> {
    if auth.origins.is_empty() {
        return Ok(());
    }
    let Some(origin) = payload_origin(payload) else {
        return Ok(());
    };
    if auth.origins.iter().any(|allowed| allowed == &origin) {
        return Ok(());
    }
    Err(format!("computer.{method}: origin `{origin}` not granted"))
}

fn payload_origin(payload: &Value) -> Option<String> {
    match payload {
        Value::Map(map) => match map.borrow().get("origin") {
            Some(Value::Str(origin)) => Some((**origin).clone()),
            _ => None,
        },
        _ => None,
    }
}
