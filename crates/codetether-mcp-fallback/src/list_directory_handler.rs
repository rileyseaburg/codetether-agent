//! Execute the fallback list_directory tool with its existing file semantics.
use super::*;
pub(super) fn call(args: Value) -> Result<CallToolResult> {
    let path = args
        .get("path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing path"))?;

    let mut entries = Vec::new();
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let name = entry.file_name().to_string_lossy().to_string();
        let suffix = if file_type.is_dir() { "/" } else { "" };
        entries.push(format!("{}{}", name, suffix));
    }

    entries.sort();

    Ok(CallToolResult {
        content: vec![ToolContent::Text {
            text: entries.join("\n"),
        }],
        is_error: false,
    })
}
