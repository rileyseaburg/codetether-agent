//! Primary MCP authorization followed by approved-alias re-evaluation.
use super::{Host, policy_args};
use anyhow::{Result, bail};
use serde_json::Value;

/// Authorize a process invocation before its MCP server is spawned.
///
/// # Arguments
/// * `command` — Executable name.
/// * `args` — Process arguments supplied to the executable.
/// * `approval_id` — Optional approval receipt to evaluate.
/// # Returns
/// Unit when the primary policy allows, or an approved alias also evaluates cleanly.
/// # Errors
/// Returns the primary policy's denial text when neither path authorizes the call.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_mcp_approval::subprocess::Host>() -> anyhow::Result<()> {
/// codetether_mcp_approval::subprocess::guard::<H>("npx", &["server"], None).await
/// # }
/// ```
pub async fn guard<H: Host>(command: &str, args: &[&str], approval_id: Option<&str>) -> Result<()> {
    let policy_args = policy_args(command, args, approval_id);
    let Some(blocked) = H::evaluate("mcp", &policy_args).await else {
        return Ok(());
    };
    if alias_approval_allows::<H>(&policy_args).await {
        return Ok(());
    }
    bail!(blocked.output)
}

async fn alias_approval_allows<H: Host>(args: &Value) -> bool {
    H::approved("mcp_bridge", args) && H::evaluate("mcp_bridge", args).await.is_none()
}
