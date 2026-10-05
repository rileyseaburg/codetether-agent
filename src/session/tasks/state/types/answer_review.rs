//! Materialized hold for an unanswered or unaccepted user question.

use crate::session::tasks::GoalStatus;

/// Persisted answer-review hold, independent of tool approval grants.
///
/// # Examples
/// ```rust
/// use codetether_agent::session::tasks::{AnswerReview, GoalStatus};
/// let review = AnswerReview {
///     id: "review-1".into(), goal_id: "goal-1".into(),
///     question: Some("Why?".into()), ready: false,
///     resume_status: GoalStatus::Active,
/// };
/// assert!(!review.ready);
/// ```
#[derive(Clone, Debug)]
pub struct AnswerReview {
    /// Identity that rejects stale answers and decisions.
    pub id: String,
    /// Goal that was interrupted.
    pub goal_id: String,
    /// Question awaiting delivery to the tool-less answer provider.
    pub question: Option<String>,
    /// Whether the answer is available for the yes/no selection.
    pub ready: bool,
    /// Goal status to restore only after an explicit Yes.
    pub resume_status: GoalStatus,
}
