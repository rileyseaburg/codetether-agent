//! Tool metadata and asynchronous execution entry point.
use super::{ConfirmMultiEditTool, Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
#[async_trait]
impl Tool for ConfirmMultiEditTool {
    fn id(&self) -> &str {
        "confirm_multiedit"
    }

    fn name(&self) -> &str {
        "Confirm Multi Edit"
    }

    fn description(&self) -> &str {
        "Edit multiple files with confirmation. Shows diffs for all changes and requires user confirmation before applying."
    }
    fn parameters(&self) -> Value {
        super::schema::parameters()
    }
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        super::execute::execute(input).await
    }
}
