use crate::tool::ToolResult;
use serde_json::{Value, json};
use std::path::Path;

pub(super) fn mark(result: ToolResult, cwd: Option<&Path>, reason: &str) -> ToolResult {
    let unsafe_execution = reason != "read_only_command";
    result
        .with_metadata("sandboxed", json!(false))
        .with_metadata("sandbox_mode", json!("unsandboxed"))
        .with_metadata("allowed_cwd", cwd_json(cwd))
        .with_metadata("network_policy", json!("host"))
        .with_metadata("unsafe_execution", json!(unsafe_execution))
        .with_metadata("unsafe_fallback_reason", json!(reason))
}

fn cwd_json(cwd: Option<&Path>) -> Value {
    cwd.map(|path| json!(path.display().to_string()))
        .unwrap_or(Value::Null)
}
