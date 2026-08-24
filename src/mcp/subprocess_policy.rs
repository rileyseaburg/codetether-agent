//! Runtime policy gate for spawning external MCP server processes.

#[path = "subprocess_policy_args.rs"]
mod args;
#[path = "subprocess_preflight.rs"]
mod preflight;
#[path = "subprocess_sandbox.rs"]
pub(crate) mod sandbox;

use anyhow::{Result, bail};

pub(super) async fn guard_scoped(
    command: &str,
    args: &[&str],
    approval_id: Option<&str>,
    network_allowed: bool,
    session_id: &str,
) -> Result<()> {
    if approval_id.is_some_and(|id| !id.trim().is_empty()) {
        preflight::executable(command)?;
        sandbox::preflight(command, args, network_allowed).await?;
    }
    let policy_args = args::scoped(command, args, approval_id, network_allowed, session_id);
    if approval_allows(&policy_args).await {
        return Ok(());
    }
    match crate::runtime_policy::evaluate_tool_invocation("mcp", &policy_args).await {
        Some(blocked) => bail!(blocked.output),
        None => Ok(()),
    }
}

#[cfg(test)]
async fn guard(command: &str, args: &[&str], approval_id: Option<&str>) -> Result<()> {
    guard_scoped(
        command,
        args,
        approval_id,
        crate::tool::network_access::allowed(),
        "mcp-test",
    )
    .await
}

async fn approval_allows(args: &serde_json::Value) -> bool {
    crate::runtime_policy::invocation::evaluate_approved_tool_invocation("mcp", args).await
        || crate::runtime_policy::invocation::evaluate_approved_tool_invocation("mcp_bridge", args)
            .await
}

#[cfg(test)]
use args::policy_args;
#[cfg(test)]
#[path = "subprocess_policy_tests.rs"]
mod tests;
