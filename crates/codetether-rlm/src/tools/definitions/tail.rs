//! Tool schema for rlm_tail.
use codetether_provider_core::ToolDefinition;
pub(super) fn definition() -> ToolDefinition {
    ToolDefinition {
        name: "rlm_tail".to_string(),
        description: "Return the last N lines of the loaded context.".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "n": {
                    "type": "integer",
                    "description": "Number of lines from the end (default: 10)"
                }
            },
            "required": []
        }),
    }
}
