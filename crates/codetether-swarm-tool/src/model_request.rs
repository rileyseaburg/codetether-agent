//! Model reference extraction from `swarm_execute` tool input.

use serde_json::Value;

/// Read an explicit model reference before the inherited parent model.
/// # Arguments
/// * `params` — Tool invocation JSON.
/// # Returns
/// The selected string, or `None` if its JSON value is not a string.
/// # Examples
/// ```
/// assert_eq!(codetether_swarm_tool::model_request::requested(&serde_json::json!({})), None);
/// ```
pub fn requested(params: &Value) -> Option<&str> {
    params
        .get("model")
        .or_else(|| params.get("__ct_current_model"))
        .and_then(Value::as_str)
}
