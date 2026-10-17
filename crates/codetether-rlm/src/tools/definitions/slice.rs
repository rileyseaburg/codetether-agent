//! Tool schema for rlm_slice.
use codetether_provider_core::ToolDefinition;
pub(super) fn definition() -> ToolDefinition {
    ToolDefinition {
        name: "rlm_slice".to_string(),
        description: "Return a slice of the context by line range.".to_string(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "start": {
                    "type": "integer",
                    "description": "Start line number (0-indexed)"
                },
                "end": {
                    "type": "integer",
                    "description": "End line number (exclusive)"
                }
            },
            "required": ["start", "end"]
        }),
    }
}
