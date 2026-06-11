//! Argument shaping for MCP bridge calls.

use serde_json::{Value, json};

pub(super) fn with_approval(arguments: Value, approval_id: Option<&str>) -> Value {
    let mut arguments = if arguments.is_null() {
        json!({})
    } else {
        arguments
    };
    if let Some(id) = approval_id
        && let Some(map) = arguments.as_object_mut()
    {
        map.insert("approval_id".to_string(), json!(id));
    }
    arguments
}
