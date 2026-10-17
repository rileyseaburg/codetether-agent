//! Tool schema for rlm_head.
use codetether_provider_core::ToolDefinition;
pub(super) fn definition() -> ToolDefinition {
    ToolDefinition {
        name: "rlm_head".to_string(),
        description: "Return the first N lines of the loaded context.".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "n": {
                    "type": "integer",
                    "description": "Number of lines from the start (default: 10)"
                }
            },
            "required": []
        }),
    }
}
