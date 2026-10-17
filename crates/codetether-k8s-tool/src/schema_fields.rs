//! schema fields for Kubernetes tool execution.
use super::*;
pub(super) fn action() -> Value {
    json!({
        "type": "string",
        "enum": ["status","list_pods","scale","rolling_restart",
                  "spawn_pod","delete_pod","pod_state","logs","recent_actions"],
        "description": "The Kubernetes action to perform."
    })
}
pub(super) fn environment() -> Value {
    json!({
        "type": "object",
        "additionalProperties": {"type": "string"},
        "description": "Environment variables for spawned sub-agent pods."
    })
}
pub(super) fn labels() -> Value {
    json!({
        "type": "object",
        "additionalProperties": {"type": "string"},
        "description": "Labels for spawned sub-agent pods."
    })
}
