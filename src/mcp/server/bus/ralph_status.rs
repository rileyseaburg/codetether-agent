//! Registration of the ralph_status MCP tool and its argument schema.
use super::*;
pub(super) async fn register(server: &McpServer) {
    server
        .register_tool(
            "ralph_status",
            "Get current Ralph PRD status: story pass/fail states, iteration count, \
             and progress.txt content. Reads prd.json and progress.txt from the \
             current working directory.",
            json!({
                "type": "object",
                "properties": {
                    "prd_path": {
                        "type": "string",
                        "description": "Path to prd.json (default: ./prd.json)"
                    }
                }
            }),
            Arc::new(super::ralph_status_handler::call),
        )
        .await;
}
