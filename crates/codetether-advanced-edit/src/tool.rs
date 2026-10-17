//! Tool metadata and asynchronous execution entry point.
use super::{AdvancedEditTool, Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
#[async_trait]
impl Tool for AdvancedEditTool {
    fn id(&self) -> &str {
        "edit"
    }

    fn name(&self) -> &str {
        "Edit"
    }

    fn description(&self) -> &str {
        "Edit a file by replacing oldString with newString. Uses multiple matching strategies \
         including exact match, line-trimmed, block anchor, whitespace normalized, and \
         indentation flexible matching. Fails if match is ambiguous."
    }
    fn parameters(&self) -> Value {
        super::schema::parameters()
    }
    async fn execute(&self, params: Value) -> Result<ToolResult> {
        super::execute::execute(params).await
    }
}
