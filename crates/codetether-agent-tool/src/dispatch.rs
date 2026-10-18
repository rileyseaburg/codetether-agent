//! Routing from parsed legacy agent actions to focused handlers.

use crate::Host;
use anyhow::{Context, Result};
use codetether_agent_input::Params;
use codetether_tool_core::ToolResult;

/// Dispatch one parsed action after hydration by the tool entrypoint.
/// # Arguments
/// * `params` — Validated shared payload with native runtime context.
/// # Returns
/// The selected native action's result or the existing unknown-action error.
/// # Errors
/// Preserves missing-target, configuration and native action errors.
/// # Examples
/// Use [crate::AgentTool] for the complete parse/hydrate/dispatch entrypoint.
pub async fn execute<H: Host>(params: &Params) -> Result<ToolResult> {
    let owner = params.parent_session_id.as_deref();
    match params.action.as_str() {
        "spawn" => H::spawn(params).await,
        "message" => H::message(params).await,
        "list" => H::list(owner).await,
        "status" => Ok(H::status(owner)),
        "interrupt" => super::actions::execute_interrupt::<H>(params).await,
        "close" => H::close(target(params, "close")?, owner).await,
        "resume" => H::resume(target(params, "resume")?, owner, params.resume_config()).await,
        "kill" => super::actions::execute_kill::<H>(params).await,
        _ => Ok(super::actions::unknown_action_result(&params.action)),
    }
}

fn target<'a>(params: &'a Params, action: &str) -> Result<&'a str> {
    params
        .name
        .as_deref()
        .with_context(|| format!("name required for {action}"))
}
