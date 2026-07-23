//! Human-readable first-party collaboration results for the tool panel.

use serde_json::Value;

mod roster;

pub(super) fn format(name: &str, output: &str) -> String {
    if name != "agent" {
        return output.to_string();
    }
    let Ok(value) = serde_json::from_str::<Value>(output) else {
        return output.to_string();
    };
    match value {
        Value::Array(peers) => roster::format(&peers),
        Value::Object(fields) => reply(&fields).unwrap_or_else(|| output.to_string()),
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => output.to_string(),
    }
}

fn reply(fields: &serde_json::Map<String, Value>) -> Option<String> {
    let agent = fields.get("agent")?.as_str()?;
    let response = fields.get("response")?.as_str()?;
    Some(format!("↔ @{agent} via mDNS\n{response}"))
}
