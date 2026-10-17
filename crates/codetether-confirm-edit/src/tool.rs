//! Tool metadata and execution entry point.
use super::{ConfirmEditTool, Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
#[async_trait]
impl Tool for ConfirmEditTool {
    fn id(&self) -> &str {
        "confirm_edit"
    }

    fn name(&self) -> &str {
        "Confirm Edit"
    }

    fn description(&self) -> &str {
        "Edit files with confirmation. Shows diff and requires user confirmation before applying changes."
    }
    fn parameters(&self) -> Value {
        super::schema::parameters()
    }
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        super::execute::execute(input).await
    }
}
