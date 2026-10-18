//! Queue fixtures and mocked local terminal snapshots for approval regressions.

use crate::approval::LiveApprovalRequest;
use crate::tui::app::state::approval_queue;
#[path = "render_support.rs"]
mod render_support;
pub(super) use render_support::draw;

pub(super) struct QueueGuard;
impl Drop for QueueGuard {
    fn drop(&mut self) {
        approval_queue::reset();
    }
}

pub(super) fn queue(preview: &str) -> approval_queue::ApprovalSnapshot {
    approval_queue::push(
        LiveApprovalRequest::new(
            "modal-test".into(),
            "call-test".into(),
            "exec_command".into(),
            "execute".into(),
            "workspace".into(),
            "reason".into(),
        )
        .with_preview(preview.into()),
    )
}
