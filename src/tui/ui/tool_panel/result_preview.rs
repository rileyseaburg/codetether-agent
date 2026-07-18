//! Human-readable first-party collaboration results for the tool panel.

use serde_json::Value;

pub(super) fn format(name: &str, output: &str) -> String {
    if name != "agent" {
        return output.to_string();
    }
    let Ok(value) = serde_json::from_str::<Value>(output) else {
        return output.to_string();
    };
    match value {
        Value::Array(peers) => roster(&peers),
        Value::Object(fields) => reply(&fields).unwrap_or_else(|| output.to_string()),
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => output.to_string(),
    }
}

fn roster(peers: &[Value]) -> String {
    let names = peers
        .iter()
        .filter_map(|peer| peer.get("name").and_then(Value::as_str))
        .take(4)
        .map(|name| format!("@{name}"))
        .collect::<Vec<_>>();
    let suffix = (peers.len() > names.len()).then(|| format!(" +{}", peers.len() - names.len()));
    format!(
        "{} collaborators ready: {}{}",
        peers.len(),
        names.join(", "),
        suffix.unwrap_or_default()
    )
}

fn reply(fields: &serde_json::Map<String, Value>) -> Option<String> {
    let agent = fields.get("agent")?.as_str()?;
    let response = fields.get("response")?.as_str()?;
    Some(format!("↔ @{agent} via mDNS\n{response}"))
}
