use serde_json::{Value, json};

use crate::tool::ToolResult;

pub fn invalid_params(tool: &str, error: serde_json::Error) -> ToolResult {
    ToolResult::structured_error(
        "invalid_params",
        tool,
        &format!("Invalid kiln_plugin params: {error}"),
        None,
        Some(example()),
    )
}

pub fn missing_source(tool: &str) -> ToolResult {
    ToolResult::structured_error(
        "missing_field",
        tool,
        "kiln_plugin requires either source or path",
        Some(vec!["source|path"]),
        Some(example()),
    )
}

fn example() -> Value {
    json!({"source": "fn validate() { return Ok(\"ok\") }", "hook": "validate"})
}
