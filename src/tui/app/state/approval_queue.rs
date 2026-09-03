//! Pending live approval queue for the TUI.

use std::collections::VecDeque;
use std::sync::{Mutex, OnceLock};

#[path = "approval_queue/edit_session.rs"]
pub(crate) mod edit_session;
#[path = "approval_queue/operations.rs"]
mod operations;
#[path = "approval_queue/queue_access.rs"]
mod queue_access;
#[path = "approval_queue/reconcile.rs"]
mod reconcile;
#[path = "approval_queue/report.rs"]
mod report;
mod snapshot;
pub(crate) use operations::{feedback_input, set_report};
#[cfg(test)]
pub(crate) use queue_access::reset;
pub(crate) use queue_access::{active, active_id, len, push, resolve};
pub(crate) use reconcile::remove_stale;
pub(crate) use report::{ApprovalReport, ApprovalReportState};
pub(crate) use snapshot::ApprovalSnapshot;

static QUEUE: OnceLock<Mutex<VecDeque<ApprovalSnapshot>>> = OnceLock::new();

fn queue() -> &'static Mutex<VecDeque<ApprovalSnapshot>> {
    QUEUE.get_or_init(|| Mutex::new(VecDeque::new()))
}
