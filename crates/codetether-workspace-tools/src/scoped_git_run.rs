//! Enforce commit capability and workspace rooting for a Git invocation.
use super::ScopedGitTool;
use anyhow::Result;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
pub(super) async fn execute(tool: &ScopedGitTool, mut args: Value) -> Result<ToolResult> {
    if !tool.allow_commit && args.get("op").and_then(Value::as_str) == Some("commit") {
        return Ok(ToolResult::error(
            "git commit is unavailable for this swarm task",
        ));
    }
    if let Some(fields) = args.as_object_mut() {
        fields.insert("cwd".into(), Value::String(tool.root.display().to_string()));
    }
    tool.inner.execute(args).await
}
