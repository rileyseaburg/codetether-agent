//! Fail-closed approval handling for unattended spawned sub-agent runs.
//!
//! Unattended children cannot turn their own approval requests into authority.

use crate::approval::LiveApprovalRequest;
use crate::approval::live::{LiveApprovalDecision, decide};

/// Deny a tool invocation that cannot be reviewed by an attached human.
///
/// # Examples
///
/// ```ignore
/// deny_unattended(&request);
/// ```
pub(super) fn deny_unattended(req: &LiveApprovalRequest) {
    let decision = match crate::approval::ApprovalStore::open_default().and_then(|store| {
        store
            .deny(
                &req.approval_id,
                "spawned-sub-agent",
                "unattended sub-agent cannot self-approve authority",
            )
            .map(|_| LiveApprovalDecision::denied_with("unattended sub-agent approval denied"))
    }) {
        Ok(decision) => decision,
        Err(error) => {
            tracing::warn!(approval_id = %req.approval_id, %error, "Sub-agent approval failed");
            LiveApprovalDecision::denied_with("sub-agent approval could not be persisted")
        }
    };
    decide(&req.approval_id, decision);
}

#[cfg(test)]
#[path = "approve_tests.rs"]
mod tests;
