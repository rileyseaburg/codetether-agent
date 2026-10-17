//! Tool schema for rlm_count.
use codetether_provider_core::ToolDefinition;
pub(super) fn definition() -> ToolDefinition {
    ToolDefinition {
        name: "rlm_count".to_string(),
        description: "Count occurrences of a regex pattern in the loaded context.".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "Regex pattern to count"
                }
            },
            "required": ["pattern"]
        }),
    }
}
