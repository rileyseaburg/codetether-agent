//! Canonical task-path attachment for successful spawn results.

use crate::tool::{ToolResult, collaboration::context::RuntimeContext};
use serde_json::{Value, json};

pub(super) fn attach_task_path(mut result: ToolResult, context: &RuntimeContext) -> ToolResult {
    if !result.success {
        return result;
    }
    let Some(owner) = context.session_id.as_deref() else { return result };
    let Ok(mut output) = serde_json::from_str::<Value>(&result.output) else { return result };
    let Some(agent_id) = output.get("agent_id").and_then(Value::as_str) else { return result };
    let Ok(Some(path)) =
        crate::tool::agent::collaboration_runtime::agent_tree::canonical(owner, agent_id)
    else {
        return result;
    };
    output["task_name"] = json!(path);
    result.output = output.to_string();
    result
}
