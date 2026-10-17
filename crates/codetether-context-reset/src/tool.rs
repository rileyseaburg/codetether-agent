//! Context-reset tool metadata and execution entry point.
use crate::ContextResetTool;
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
#[async_trait]
impl Tool for ContextResetTool {
    fn id(&self) -> &str {
        "context_reset"
    }

    fn name(&self) -> &str {
        "ContextReset"
    }

    fn description(&self) -> &str {
        "REQUEST A CONTEXT RESET (Lu et al., arXiv:2510.06727). Call this \
         when the active transcript is approaching the model's context \
         window and you want to compress it in your own words before \
         continuing. Pass a `summary` string distilling the task state, \
         decisions, open questions, and any must-remember details. The \
         tool records the reset and returns a `[CONTEXT RESET]` marker \
         that will become the anchor for the next turn. The canonical \
         chat history is preserved on disk — use `session_recall` only \
         when the user allows history access."
    }
    fn parameters(&self) -> Value {
        super::schema::parameters()
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        super::execute::run(args).await
    }
}
