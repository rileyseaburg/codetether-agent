//! Agent dispatch after an enclosing tool has already claimed authority.

use super::params::Params;
use crate::tool::ToolResult;
use anyhow::{Context, Result};
use serde_json::Value;

impl super::tool_impl::AgentTool {
    pub(crate) async fn execute_authorized(value: Value) -> Result<ToolResult> {
        let params = serde_json::from_value(value).context("Invalid params")?;
        execute(params).await
    }
}

pub(super) async fn execute(params: Params) -> Result<ToolResult> {
    super::persistence::hydrate_parent(params.parent_session_id.as_deref()).await?;
    super::dispatch::execute(&params).await
}