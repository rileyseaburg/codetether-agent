//! Wait dispatch for application-level integration tests.
use crate::{WaitArgs, WaitHost};
use anyhow::Result;
use codetether_tool_core::ToolResult;
/// Execute a decoded collaboration wait with the application's notification host.
/// # Arguments
/// * `args` — Parent context, timeout and optional legacy targets.
/// # Returns
/// A mailbox or status response with the original timeout metadata.
/// # Errors
/// Propagates missing-owner, timeout-validation or target-resolution errors.
/// # Examples
/// Use [crate::WaitAgentTool] to decode JSON and dispatch a wait.
pub async fn execute_wait<H: WaitHost>(args: WaitArgs) -> Result<ToolResult> {
    crate::wait::run::execute::<H>(args).await
}
