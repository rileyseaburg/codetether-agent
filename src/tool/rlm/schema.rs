//! JSON schema for the RLM tool.

use serde_json::{Value, json};

pub(super) const DESCRIPTION: &str = "Recursive Language Model for processing large codebases. Use this when you need to analyze files or content that exceeds the context window. RLM chunks the content, processes each chunk, and synthesizes results. Actions: 'analyze' (analyze large content), 'summarize' (summarize large files), 'search' (semantic search across large codebase).";

pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": {
                "type": "string",
                "description": "Action: 'analyze' (deep analysis), 'summarize' (generate summary), 'search' (semantic search)",
                "enum": ["analyze", "summarize", "search"]
            },
            "query": {
                "type": "string",
                "description": "The question or query to answer (for analyze/search)"
            },
            "paths": {
                "type": "array",
                "items": {"type": "string"},
                "description": "File or directory paths to process"
            },
            "content": {
                "type": "string",
                "description": "Direct content to analyze (alternative to paths)"
            },
            "max_depth": {
                "type": "integer",
                "description": "Maximum recursion depth (default: 3)",
                "default": 3
            }
        },
        "required": ["action"]
    })
}
