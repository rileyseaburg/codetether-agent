//! Schema and registration for the fallback run_command tool.
use super::*;
pub(super) async fn register(server: &McpServer) {
    server
        .register_tool(
            "run_command",
            "Execute a shell command and return the output",
            json!({
                "type": "object",
                "properties": {
                    "command": {
                        "type": "string",
                        "description": "The command to execute"
                    },
                    "cwd": {
                        "type": "string",
                        "description": "Working directory (optional)"
                    },
                    "timeout_ms": {
                        "type": "integer",
                        "description": "Timeout in milliseconds (default: 30000)"
                    },
                    "approval_id": {
                        "type": "string",
                        "description": "Approval receipt id for mutating command execution"
                    }
                },
                "required": ["command"]
            }),
            Arc::new(super::super::super::fallback_run_command::call),
        )
        .await;
}
