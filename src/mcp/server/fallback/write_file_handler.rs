//! Execute the fallback write_file tool with its existing file semantics.
use super::*;
pub(super) fn call(args: Value) -> Result<CallToolResult> {
    if let Some(blocked) = super::super::super::tool_policy::blocked("write_file", &args) {
        return Ok(blocked);
    }
    let path = args
        .get("path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing path"))?;

    let content = args
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing content"))?;

    let create_dirs = args
        .get("create_dirs")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if create_dirs && let Some(parent) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(path, content)?;

    Ok(CallToolResult {
        content: vec![ToolContent::Text {
            text: format!("Wrote {} bytes to {}", content.len(), path),
        }],
        is_error: false,
    })
}
