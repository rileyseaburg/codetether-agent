//! Display successful or failed MCP results using the existing content policy.
use codetether_mcp_core::types as mcp;
pub(super) fn print(result: &mcp::CallToolResult) {
    let output: String = result
        .content
        .iter()
        .map(|c| match c {
            mcp::ToolContent::Text { text } => text.clone(),
            mcp::ToolContent::Image { data, mime_type } => {
                format!("[image: {} ({} bytes)]", mime_type, data.len())
            }
            mcp::ToolContent::Resource { resource } => {
                serde_json::to_string(resource).unwrap_or_default()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    if result.is_error {
        eprintln!("Error: {}", output);
    } else {
        println!("{}", output);
    }
}
