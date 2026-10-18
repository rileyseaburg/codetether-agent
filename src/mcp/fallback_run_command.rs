//! Execute translated fallback command arguments through the native sandbox.
use crate::tool::{Tool, ToolResult, bash::BashTool};
use anyhow::Result;
use serde_json::Value;
pub(super) fn execute(bash_args: Value) -> Result<ToolResult> {
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current()
            .block_on(async { BashTool::new().execute(bash_args).await })
    })
}
