//! Resolve direct or provenance-nested agent names.

use serde_json::Value;

pub(super) fn agent_name(value: &Value) -> Option<&str> {
    value.get("agent").and_then(Value::as_str).or_else(|| {
        value
            .pointer("/metadata/provenance/identity/agent_name")
            .and_then(Value::as_str)
    })
}
