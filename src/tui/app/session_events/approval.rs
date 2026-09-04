//! Approval request event rendering.

#[path = "approval_analysis.rs"]
mod analysis;

use crate::approval::LiveApprovalRequest;
use crate::tui::app::state::{App, approval_queue};

pub(super) fn request(app: &mut App, request: LiveApprovalRequest) {
    app.state.approval_preview_scroll = 0;
    app.state.approval_waiting = true;
    let pending = approval_queue::push(request);
    analysis::start(app, &pending);
    let count = approval_queue::len();
    let guidance = crate::tui::ui::trust_status::approval_guidance();
    let because = pending
        .justification
        .as_deref()
        .map(|text| format!(" because: {text}."))
        .unwrap_or_default();
    let text = format!(
        "Approval pending ({count}): `{}` wants to {} — {}.{because} (key `{}`) Ctrl+A approves, Ctrl+D denies. Slash: `/approve {}` or `/deny {}`. {guidance}",
        pending.tool, pending.action, pending.reason, pending.resource, pending.id, pending.id
    );
    app.state.status = text;
}

#[cfg(test)]
#[path = "approval_tests.rs"]
mod tests;
