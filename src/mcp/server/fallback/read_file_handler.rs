//! Execute the fallback read_file tool with its existing file semantics.
use super::*;
pub(super) fn call(args: Value) -> Result<CallToolResult> {
    let path = args
        .get("path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing path"))?;

    let content = std::fs::read_to_string(path)?;

    let offset = args.get("offset").and_then(|v| v.as_u64()).unwrap_or(1) as usize;
    let limit = args.get("limit").and_then(|v| v.as_u64());

    let lines: Vec<&str> = content.lines().collect();
    let start = (offset.saturating_sub(1)).min(lines.len());
    let end = if let Some(l) = limit {
        (start + l as usize).min(lines.len())
    } else {
        lines.len()
    };

    let result = lines[start..end].join("\n");

    Ok(CallToolResult {
        content: vec![ToolContent::Text { text: result }],
        is_error: false,
    })
}
