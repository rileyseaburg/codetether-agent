//! Schema and registration for the fallback write_file tool.
use super::*;
pub(super) async fn register(server: &McpServer) {
    server
        .register_tool(
            "write_file",
            "Write content to a file",
            json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Path to the file to write"
                    },
                    "content": {
                        "type": "string",
                        "description": "Content to write"
                    },
                    "create_dirs": {
                        "type": "boolean",
                        "description": "Create parent directories if they don't exist"
                    },
                    "approval_id": {
                        "type": "string",
                        "description": "Approval receipt id for file writes"
                    }
                },
                "required": ["path", "content"]
            }),
            Arc::new(super::write_file_handler::call),
        )
        .await;
}
