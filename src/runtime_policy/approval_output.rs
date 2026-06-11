//! Approval request fields embedded in structured tool errors.

use crate::tool::ToolResult;
use serde_json::{Value, json};

pub(super) fn with_request(
    mut result: ToolResult,
    request_id: &str,
    action: &str,
    resource: &str,
) -> ToolResult {
    let Ok(mut output) = serde_json::from_str::<Value>(&result.output) else {
        return result;
    };
    let Some(error) = output.get_mut("error").and_then(Value::as_object_mut) else {
        return result;
    };
    error.insert("approval_request_id".to_string(), json!(request_id));
    error.insert("approval_action".to_string(), json!(action));
    error.insert("approval_resource".to_string(), json!(resource));
    error.insert("missing_fields".to_string(), json!(["approval_id"]));
    error.insert("example".to_string(), json!({ "approval_id": request_id }));
    error.insert(
        "why".to_string(),
        json!("current access/approval policy requires approval for this tool"),
    );
    error.insert(
        "change_prompting".to_string(),
        json!(
            "use `codetether config --set access_mode=approve` for fewer prompts or `access_mode=full` for none"
        ),
    );
    if let Ok(rendered) = serde_json::to_string_pretty(&output) {
        result.output = rendered;
    }
    result
}
