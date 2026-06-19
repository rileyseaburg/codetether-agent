//! Autonomous approval handling for spawned sub-agent runs.
//!
//! Sub-agents run with no human attached to the event channel, so an
//! [`crate::session::SessionEvent::ApprovalRequest`] would otherwise block the
//! sub-agent's `live::request` oneshot forever. This module answers such
//! requests immediately so autonomous writes/commands proceed.

use crate::approval::LiveApprovalRequest;
use crate::approval::live::{LiveApprovalDecision, decide};

/// Auto-approve a tool invocation requested by a spawned sub-agent.
///
/// # Examples
///
/// ```ignore
/// auto_approve(&request);
/// ```
pub(super) fn auto_approve(req: &LiveApprovalRequest) {
    decide(&req.approval_id, LiveApprovalDecision::Approved);
}
