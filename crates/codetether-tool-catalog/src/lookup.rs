//! Fetch a full schema with case-insensitive tool-name matching.
use codetether_provider_core::ToolDefinition;
use serde_json::json;
pub(super) fn lookup(tools: &[ToolDefinition], name: &str) -> String {
    if let Some(found) = tools.iter().find(|t| t.name.eq_ignore_ascii_case(name)) {
        return serde_json::to_string_pretty(&json!({
            "mode": "tool",
            "tool": {
                "name": found.name,
                "description": found.description,
                "parameters": found.parameters
            }
        }))
        .unwrap_or_else(|_| format!("tool: {}", found.name));
    }
    return serde_json::to_string_pretty(&json!({
        "mode": "tool",
        "error": format!("Tool '{}' not found", name)
    }))
    .unwrap_or_else(|_| format!("Tool '{}' not found", name));
}
