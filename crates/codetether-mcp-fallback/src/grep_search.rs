//! Schema and registration for the fallback grep_search tool.
use super::*;
pub(super) async fn register<H: Host>(server: &ServerCatalog) {
    server
        .register_tool(
            "grep_search",
            "Search file contents using grep",
            json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Search pattern"
                    },
                    "path": {
                        "type": "string",
                        "description": "Directory or file to search"
                    },
                    "is_regex": {
                        "type": "boolean",
                        "description": "Treat pattern as regex"
                    },
                    "case_sensitive": {
                        "type": "boolean",
                        "description": "Case-sensitive search"
                    }
                },
                "required": ["query"]
            }),
            Arc::new(H::grep_search),
        )
        .await;
}
