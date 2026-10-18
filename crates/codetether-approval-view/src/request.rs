//! Presentation and background checks for incoming approval requests.
//!
//! [request] enqueues a request, starts analysis/review, then formats its status.
use codetether_approval_core::interaction::LiveApprovalRequest;
use codetether_approval_queue::{self as approval_queue, ApprovalSnapshot};
/// Frontend effects surrounding a newly queued approval.
/// # Implementors
/// Frontends supply diagnostics/reviewer context and current trust guidance.
pub trait View {
    /// Reset preview scrolling for the new request.
    fn reset_preview_scroll(&mut self);
    /// Set whether the frontend is awaiting approval.
    fn set_waiting(&mut self, waiting: bool);
    /// Start proposed-content diagnostics.
    fn start_analysis(&mut self, item: &ApprovalSnapshot);
    /// Start the configured approval reviewer.
    fn start_review(&mut self, item: &ApprovalSnapshot);
    /// Explain the current approval and sandbox policy.
    fn approval_guidance(&self) -> String;
    /// Replace status text.
    fn set_status(&mut self, status: String);
}
/// Enqueue and present an approval request without adding a transcript message.
/// # Arguments
/// * `view` — Frontend state and background-check operations.
/// * `request` — Pending live tool request.
/// # Returns
/// Updates queue/presentation state and starts analysis before review.
/// # Examples
/// ```no_run
/// # fn example(view: &mut impl codetether_approval_view::request::View, request: codetether_approval_core::interaction::LiveApprovalRequest) {
/// codetether_approval_view::request::request(view, request);
/// # }
/// ```
pub fn request(view: &mut impl View, request: LiveApprovalRequest) {
    view.reset_preview_scroll();
    view.set_waiting(true);
    let pending = approval_queue::push(request);
    view.start_analysis(&pending);
    view.start_review(&pending);
    let count = approval_queue::len();
    let guidance = view.approval_guidance();
    let because = pending
        .justification
        .as_deref()
        .map(|text| format!(" because: {text}."))
        .unwrap_or_default();
    let text = format!(
        "Approval pending ({count}): `{}` wants to {} — {}.{because} (key `{}`) Ctrl+A approves, Ctrl+D denies. Slash: `/approve {}` or `/deny {}`. {guidance}",
        pending.tool, pending.action, pending.reason, pending.resource, pending.id, pending.id
    );
    view.set_status(text);
}
