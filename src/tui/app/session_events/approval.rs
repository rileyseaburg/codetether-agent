//! Native approval presentation and reviewer context bindings.
#[path = "approval_host.rs"]
mod host;
#[path = "approval_review.rs"]
mod review;
use crate::{approval::LiveApprovalRequest, tui::app::state::App};
pub(super) fn request(app: &mut App, request: LiveApprovalRequest) {
    codetether_approval_view::request::request(&mut host::Native(app), request);
}
#[cfg(test)]
#[path = "approval_tests.rs"]
mod tests;
