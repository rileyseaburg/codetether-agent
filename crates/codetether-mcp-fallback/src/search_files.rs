//! Schema and registration for the fallback search_files tool.
use super::*;
pub(super) async fn register<H: Host>(server: &ServerCatalog) {
    server
        .register_tool(
            "search_files",
            "Search for files matching a pattern",
            json!({
                "type": "object",
                "properties": {
                    "pattern": {
                        "type": "string",
                        "description": "Search pattern (glob or regex)"
                    },
                    "path": {
                        "type": "string",
                        "description": "Directory to search in"
                    },
                    "content_pattern": {
                        "type": "string",
                        "description": "Pattern to search in file contents"
                    }
                },
                "required": ["pattern"]
            }),
            Arc::new(H::search_files),
        )
        .await;
}
