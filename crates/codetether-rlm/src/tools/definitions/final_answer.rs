//! Tool schema for rlm_final.
use codetether_provider_core::ToolDefinition;
pub(super) fn definition() -> ToolDefinition {
    ToolDefinition {
            name: "rlm_final".to_string(),
            description: "Return the final structured payload to the analysis query. Always emit a JSON payload matching the FINAL schema.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "payload": {
                        "type": "object",
                        "description": "FINAL(JSON) payload. Preferred over `answer`."
                    },
                    "answer": {
                        "type": "string",
                        "description": "Deprecated compatibility field; if used, must contain the FINAL(JSON) payload string."
                    }
                },
                "additionalProperties": false
            }),
        }
}
