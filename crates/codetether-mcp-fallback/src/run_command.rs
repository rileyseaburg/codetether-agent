//! Schema and registration for the fallback run_command tool.
use super::*;
pub(super) async fn register<H: Host>(server: &ServerCatalog) {
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
            Arc::new(H::run_command),
        )
        .await;
}
