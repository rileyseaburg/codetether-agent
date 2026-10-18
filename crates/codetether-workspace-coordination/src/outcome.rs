//! Translation of coordinator replies into central tool-gate outcomes.

use codetether_worktree_lease::CoordinationReply;

pub(super) fn classify(
    tool: &str,
    agent: &str,
    reply: CoordinationReply,
) -> Option<codetether_tool_reporting::ToolTuple> {
    match reply {
        CoordinationReply::WorkspaceScopeForbidden => Some(super::scope_error::result(tool)),
        CoordinationReply::Acquired { leases, waited_ms } => {
            tracing::info!(target: "codetether_agent::session::helper::workspace_coordination::outcome",
                agent = %agent,
                count = leases.len(),
                waited_ms,
                "Mux worktree lease acquired"
            );
            None
        }
        CoordinationReply::Blocked {
            conflicts,
            waited_ms,
            retry_after_ms,
        } => {
            tracing::warn!(target: "codetether_agent::session::helper::workspace_coordination::outcome",
                agent = %agent,
                count = conflicts.len(),
                waited_ms,
                "Mux lease wait timed out"
            );
            Some(super::gate_error::conflict(
                tool,
                &conflicts,
                waited_ms,
                retry_after_ms,
            ))
        }
        CoordinationReply::Renewed { .. }
        | CoordinationReply::Released { .. }
        | CoordinationReply::Snapshot { .. } => Some(super::gate_failure::invalid(tool)),
    }
}
