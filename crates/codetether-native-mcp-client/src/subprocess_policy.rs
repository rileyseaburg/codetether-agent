//! Native entry point for shared external MCP process authorization.
use anyhow::Result;
#[cfg(test)]
use codetether_mcp_approval::subprocess::policy_args;

pub(super) async fn guard(command: &str, args: &[&str], approval_id: Option<&str>) -> Result<()> {
    codetether_mcp_approval::subprocess::guard::<super::policy_host::Native>(
        command,
        args,
        approval_id,
    )
    .await
}

#[cfg(test)]
#[path = "subprocess_policy_tests.rs"]
mod tests;
