//! Native integration for the shared pending approval display queue.
#[path = "approval_queue/edit_session.rs"]
pub(crate) mod edit_session;
#[path = "approval_queue/reconcile.rs"]
mod reconcile;
#[cfg(test)]
pub(crate) use codetether_approval_queue::reset;
pub(crate) use codetether_approval_queue::{
    ApprovalReport, ApprovalReportState, ApprovalSnapshot, active, active_id, cycle,
    feedback_input, len, push, resolve, set_report, set_review, set_scroll_limit,
};
pub(crate) use reconcile::remove_stale;
