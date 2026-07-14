//! Model reference extraction from `swarm_execute` tool input.

use serde_json::Value;

pub(super) fn requested(params: &Value) -> Option<&str> {
    params
        .get("model")
        .or_else(|| params.get("__ct_current_model"))
        .and_then(Value::as_str)
}
