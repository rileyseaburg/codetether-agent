//! Exact-once approval boundary for LSP server operations.

use crate::tool::ToolResult;
use anyhow::{Result, anyhow};
use serde_json::Value;

pub(super) async fn result(args: &Value) -> Result<Option<ToolResult>> {
    if let Some(blocked) = crate::runtime_policy::evaluate_tool_invocation("lsp", args).await {
        return Ok(Some(blocked));
    }
    crate::approval::use_once::claim("lsp", args)
        .map_err(|error| anyhow!("approval claim failed: {error}"))?;
    Ok(None)
}
// Exact-once approval guard macro.

macro_rules! guard {
    ($args:expr) => {
        if let Some(blocked) = $crate::tool::lsp::authorize::result($args).await? {
            return Ok(blocked);
        }
    };
}

pub(super) use guard;

#[cfg(test)]
#[path = "lsp_authorize_process_tests.rs"]
mod process_tests;
#[cfg(test)]
#[path = "lsp_authorize_tests.rs"]
mod tests;
