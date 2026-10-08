//! Durable user decisions for an answer that interrupted a goal.

/// A user-question review transition; acceptance or explicit continuation releases the hold.
///
/// # Examples
/// ```rust
/// use codetether_agent::session::tasks::AnswerReviewAction;
/// let decision = AnswerReviewAction::Satisfied;
/// assert!(matches!(decision, AnswerReviewAction::Satisfied));
/// ```
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum AnswerReviewAction {
    /// Stop the goal and retain the question until its answer is delivered.
    Begin { question: String },
    /// The answer is visible and needs an explicit yes/no decision.
    Answered,
    /// Keep the goal stopped and allow a follow-up question.
    Unsatisfied,
    /// The user explicitly selected Yes; restore the prior goal status.
    Satisfied,
    /// The user explicitly requested continuation, without rating the answer.
    ResumeRequested,
}
