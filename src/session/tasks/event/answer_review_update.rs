//! A durable, identity-scoped answer-review transition.

use super::AnswerReviewAction;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// One question or explicit user satisfaction decision.
///
/// # Examples
/// ```
/// use codetether_agent::session::tasks::{AnswerReviewAction, AnswerReviewUpdate};
/// let update = AnswerReviewUpdate {
///     at: chrono::Utc::now(), goal_id: "goal".into(), review_id: "review".into(),
///     decision: AnswerReviewAction::Satisfied,
/// };
/// assert_eq!(update.goal_id, "goal");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerReviewUpdate {
    /// Time of the question or decision.
    pub at: DateTime<Utc>,
    /// Goal to which this question belongs.
    pub goal_id: String,
    /// Review identity; a later question supersedes older decisions.
    pub review_id: String,
    /// The question, answer delivery or explicit Yes/No decision.
    pub decision: AnswerReviewAction,
}
