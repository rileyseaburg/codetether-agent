//! Tool schema for rlm_grep.
use codetether_provider_core::ToolDefinition;
pub(super) fn definition() -> ToolDefinition {
    ToolDefinition {
            name: "rlm_grep".to_string(),
            description: "Search the loaded context for lines matching a regex pattern. Returns matching lines with line numbers.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "pattern": {
                        "type": "string",
                        "description": "Regex pattern to search for"
                    }
                },
                "required": ["pattern"]
            }),
        }
}
