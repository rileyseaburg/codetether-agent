//! Translation of coordinator replies into central tool-gate outcomes.

use crate::mux::lease::CoordinationReply;

pub(super) fn classify(
    tool: &str,
    agent: &str,
    reply: CoordinationReply,
) -> Option<super::super::tool_policy::ToolTuple> {
    match reply {
        CoordinationReply::Acquired { leases, waited_ms } => {
            tracing::info!(
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
            tracing::warn!(
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
