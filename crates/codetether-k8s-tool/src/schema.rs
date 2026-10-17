//! schema for Kubernetes tool execution.
use super::*;

pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": schema_fields::action(),
            "namespace": {
                "type": "string",
                "description": "Namespace override for status, list_pods, scale, rolling_restart, spawn_pod, delete_pod, pod_state, and logs. Ignored by recent_actions."
            },
            "deployment": {
                "type": "string",
                "description": "Deployment override for status, scale, rolling_restart, and list_pods only. Ignored by pod and recent_actions operations."
            },
            "replicas": {
                "type": "integer",
                "description": "Number of replicas (required for scale)."
            },
            "subagent_id": {
                "type": "string",
                "description": "Sub-agent identifier (for spawn_pod, delete_pod, pod_state, logs)."
            },
            "image": {
                "type": "string",
                "description": "Container image for spawned sub-agent pods."
            },
            "env_vars": schema_fields::environment(),
            "labels": schema_fields::labels(),
            "command": {
                "type": "array", "items": {"type": "string"},
                "description": "Command override for spawned sub-agent pods."
            },
            "args": {
                "type": "array", "items": {"type": "string"},
                "description": "Args override for spawned sub-agent pods."
            },
            "tail_lines": {
                "type": "integer",
                "description": "Number of tail lines for logs (default: 100).",
                "default": 100
            },
            "label_selector": {
                "type": "string",
                "description": "Label selector for list_pods (e.g. 'app=myapp')."
            }
        },
        "required": ["action"]
    })
}
