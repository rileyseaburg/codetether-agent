//! Fail-closed lease acquisition immediately before a mutating tool call.

use crate::mux::lease::CoordinationReply;
use serde_json::Value;

pub(in crate::session::helper) async fn blocked(
    tool: &str,
    input: &Value,
) -> Option<super::super::tool_policy::ToolTuple> {
    if !crate::mux::coordination::active() {
        return None;
    }
    let paths = super::paths::mutation_paths(tool, input)?;
    let context = match super::context::RuntimeContext::from_input(input) {
        Ok(context) => context,
        Err(error) => return Some(super::gate_failure::unavailable(tool, &error)),
    };
    let reply = crate::mux::coordination::acquire(
        &context.owner,
        &context.agent,
        &context.workspace,
        paths,
    )
    .await;
    match reply {
        Ok(Some(CoordinationReply::Acquired { leases })) => {
            tracing::info!(agent = %context.agent, count = leases.len(), "Mux worktree lease acquired");
            None
        }
        Ok(Some(CoordinationReply::Blocked { conflicts })) => {
            tracing::warn!(agent = %context.agent, count = conflicts.len(), "Mux blocked conflicting mutation");
            Some(super::gate_error::conflict(tool, &conflicts))
        }
        Ok(Some(_)) => Some(super::gate_failure::invalid(tool)),
        Ok(None) => Some(super::gate_failure::missing(tool)),
        Err(error) => Some(super::gate_failure::unavailable(tool, &error)),
    }
}
