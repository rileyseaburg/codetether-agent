//! Schema and registration for the fallback list_directory tool.
use super::*;
pub(super) async fn register(server: &McpServer) {
    server
        .register_tool(
            "list_directory",
            "List contents of a directory",
            json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Path to the directory"
                    },
                    "recursive": {
                        "type": "boolean",
                        "description": "List recursively"
                    },
                    "max_depth": {
                        "type": "integer",
                        "description": "Maximum depth for recursive listing"
                    }
                },
                "required": ["path"]
            }),
            Arc::new(super::list_directory_handler::call),
        )
        .await;
}
