//! Tool schema for rlm_llm_query.
use codetether_provider_core::ToolDefinition;
pub(super) fn definition() -> ToolDefinition {
    ToolDefinition {
            name: "rlm_llm_query".to_string(),
            description: "Ask a focused sub-question about a portion of the context. Use this for semantic understanding of specific sections.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The question to answer about the context"
                    },
                    "context_slice": {
                        "type": "string",
                        "description": "Optional: specific text slice to analyze (if omitted, uses full context)"
                    }
                },
                "required": ["query"]
            }),
        }
}
