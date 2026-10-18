//! Process-wide display queue for pending interactive approvals.
//!
//! push stores a request snapshot; active and cycle expose its display order.
//! Report/review updates annotate still-pending requests. resolve and reset
//! only remove display state: approval decisions belong to the caller.
use std::collections::VecDeque;
use std::sync::{Mutex, OnceLock};
mod navigation;
mod operations;
mod queue_access;
mod reconcile;
mod report;
mod snapshot;
mod viewport;
pub use navigation::cycle;
pub use operations::{feedback_input, set_report, set_review};
pub use queue_access::{active, active_id, len, push, reset, resolve};
pub use reconcile::remove_stale;
pub use report::{ApprovalReport, ApprovalReportState};
pub use snapshot::ApprovalSnapshot;
pub use viewport::set_scroll_limit;
static QUEUE: OnceLock<Mutex<VecDeque<ApprovalSnapshot>>> = OnceLock::new();
fn queue() -> &'static Mutex<VecDeque<ApprovalSnapshot>> {
    QUEUE.get_or_init(|| Mutex::new(VecDeque::new()))
}

/// Application-local typed helper declarations.
pub mod reconcile_binding;
