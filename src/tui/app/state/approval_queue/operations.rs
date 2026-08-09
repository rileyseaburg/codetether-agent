//! Approval queue updates and feedback-composition detection.

use super::{ApprovalReport, active_id, queue};

pub(crate) fn feedback_input(input: &str) -> bool {
    active_id().is_some() && !input.trim().is_empty()
}

pub(crate) fn set_report(id: &str, report: ApprovalReport) {
    if let Some(item) = queue()
        .lock()
        .expect("approval queue lock")
        .iter_mut()
        .find(|item| item.id == id)
    {
        item.report = report;
    }
}
