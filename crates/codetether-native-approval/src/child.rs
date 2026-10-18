//! Existing approval handling for autonomous child-agent event streams.

use crate::LiveApprovalRequest;
use crate::live::{LiveApprovalDecision, decide};

/// Approve a request received by the autonomous child event collector.
///
/// Child collectors have no attached human to answer the request. This retains
/// their existing immediate decision behavior through the normal live channel.
///
/// # Arguments
/// * `req` — Live request emitted by the child session.
/// # Returns
/// Unit after submitting the approved decision to the live approval channel.
/// # Examples
/// ```no_run
/// # fn example(req: &codetether_native_approval::LiveApprovalRequest) {
/// codetether_native_approval::approve_autonomous_child(req);
/// # }
/// ```
pub fn approve_autonomous_child(req: &LiveApprovalRequest) {
    decide(&req.approval_id, LiveApprovalDecision::Approved);
}
