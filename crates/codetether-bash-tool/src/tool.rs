//! Tool registry metadata and execution entry point.
use super::{BashTool, Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
#[async_trait]
impl Tool for BashTool {
    fn id(&self) -> &str {
        "bash"
    }

    fn name(&self) -> &str {
        "Bash"
    }

    fn description(&self) -> &str {
        "bash(command: string, cwd?: string, timeout?: int) - Execute a noninteractive shell command. Password prompts are disabled; use noninteractive credentials or flags such as sudo -n."
    }
    fn parameters(&self) -> Value {
        super::parameters::parameters()
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        super::execute::execute(self, args).await
    }
}
