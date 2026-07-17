//! [`Tool`](crate::tool::Tool) implementation for application MCP access.

use super::{ApplicationMcpTool, request, schema};
use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
impl Tool for ApplicationMcpTool {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn parameters(&self) -> Value {
        schema::parameters()
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        request::run(self, args).await
    }
}
