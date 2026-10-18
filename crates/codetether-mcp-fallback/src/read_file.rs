//! Schema and registration for the fallback read_file tool.
use super::*;
pub(super) async fn register(server: &ServerCatalog) {
    server
        .register_tool(
            "read_file",
            "Read the contents of a file",
            json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Path to the file to read"
                    },
                    "offset": {
                        "type": "integer",
                        "description": "Line offset to start reading from (1-indexed)"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of lines to read"
                    }
                },
                "required": ["path"]
            }),
            Arc::new(super::read_file_handler::call),
        )
        .await;
}
