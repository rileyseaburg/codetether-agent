//! Native review storage and interactive state operations.
use anyhow::Result;
use codetether_goal_core::{AnswerReview, AnswerReviewAction};
use std::future::Future;

/// Storage and presentation effects for a durable answer review.
///
/// # Implementors
/// Applications load the current session review and persist decisions through
/// their existing goal runtime before applying UI updates.
pub trait Host {
    /// Read the current review, propagating any storage failure.
    fn read_review(&self) -> Result<Option<AnswerReview>>;
    /// Current composer input, used to preserve command access.
    fn input(&self) -> &str;
    /// Borrow the current Yes/No selection.
    fn selected_yes(&mut self) -> &mut bool;
    /// Persist the decision for the supplied review before updating the UI.
    fn record(
        &mut self,
        review: &AnswerReview,
        action: AnswerReviewAction,
    ) -> impl Future<Output = Result<()>>;
    /// Clear composer input after a successful decision.
    fn clear_input(&mut self);
    /// Replace the displayed status after a successful decision.
    fn set_status(&mut self, status: String);
    /// Request a redraw after selection or decision handling.
    fn redraw(&mut self);
}
