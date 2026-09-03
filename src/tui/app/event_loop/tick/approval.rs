//! Reconciliation of approvals resolved outside the local TUI.

use crate::tui::app::state::{App, approval_queue};

pub(super) fn reconcile(app: &mut App) -> bool {
    let removed = approval_queue::remove_stale();
    if removed == 0 {
        return false;
    }
    let remaining = approval_queue::len();
    app.state.approval_waiting = remaining > 0;
    app.state.approval_preview_scroll = 0;
    app.state.status = if remaining == 0 {
        format!("Reconciled {removed} externally resolved approval request(s)")
    } else {
        format!("Reconciled {removed} externally resolved approval request(s); {remaining} pending")
    };
    true
}

#[cfg(test)]
#[path = "approval_tests.rs"]
mod tests;
