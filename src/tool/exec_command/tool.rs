//! Tool-registry interface for persistent command execution.

use super::{ExecCommandTool, execute, parameters};
use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
impl Tool for ExecCommandTool {
    fn id(&self) -> &str {
        "exec_command"
    }
    fn name(&self) -> &str {
        "Exec Command"
    }
    fn description(&self) -> &str {
        "Runs a command, returning output or a session ID for ongoing interaction."
    }
    fn parameters(&self) -> serde_json::Value {
        parameters::schema()
    }
    async fn execute(&self, args: serde_json::Value) -> Result<ToolResult> {
        execute::run(self, args).await
    }
}
