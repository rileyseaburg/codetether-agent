//! `Tool` trait adapter for local-first session recall.

use anyhow::Result;
use async_trait::async_trait;

use codetether_tool_core::{Tool, ToolResult};

use super::tool_struct::SessionRecallTool;

#[async_trait]
impl<H: crate::Host> Tool for SessionRecallTool<H> {
    fn id(&self) -> &str {
        "session_recall"
    }

    fn name(&self) -> &str {
        "SessionRecall"
    }

    fn description(&self) -> &str {
        super::description::DESCRIPTION
    }

    fn parameters(&self) -> serde_json::Value {
        super::schema::parameters()
    }

    async fn execute(&self, args: serde_json::Value) -> Result<ToolResult> {
        super::execute::run(self, args).await
    }
}
