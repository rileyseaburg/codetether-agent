//! Approval queue updates and feedback-composition detection.

use super::{ApprovalReport, active_id, queue};

/// Return whether nonblank input is feedback for a displayed request.
/// # Arguments
/// * `input` - Current composer text.
/// # Returns
/// True if a request is displayed and input is nonblank.
/// # Panics
/// Panics if the process-wide queue mutex was poisoned.
/// # Examples
/// ```no_run
/// let _ = codetether_approval_queue::feedback_input("feedback");
/// ```
pub fn feedback_input(input: &str) -> bool {
    active_id().is_some() && !input.trim().is_empty()
}

/// Store analysis results for a still-queued request.
/// # Arguments
/// * `id` - Request to update.
/// * `report` - Analysis state and messages.
/// # Returns
/// Updates a matching snapshot when present.
/// # Panics
/// Panics if the process-wide queue mutex was poisoned.
/// # Examples
/// ```no_run
/// codetether_approval_queue::set_report("id", Default::default());
/// ```
pub fn set_report(id: &str, report: ApprovalReport) {
    if let Some(item) = queue()
        .lock()
        .expect("approval queue lock")
        .iter_mut()
        .find(|item| item.id == id)
    {
        item.report = report;
    }
}

/// Record reviewer progress: `None` marks it running, `Some` stores the verdict.
/// Store a verdict, or mark a still-queued request as under review.
/// # Arguments
/// * `id` - Request to update.
/// * `verdict` - Completed verdict, or None while running.
/// # Returns
/// Updates a matching snapshot when present.
/// # Panics
/// Panics if the process-wide queue mutex was poisoned.
/// # Examples
/// ```no_run
/// codetether_approval_queue::set_review("id", None);
/// ```
pub fn set_review(id: &str, verdict: Option<codetether_approval_core::review::ReviewVerdict>) {
    if let Some(item) = queue()
        .lock()
        .expect("approval queue lock")
        .iter_mut()
        .find(|item| item.id == id)
    {
        item.review = Some(verdict);
    }
}
