//! Native session specialization of user-only answer-review operations.
use crate::tasks::AnswerReviewAction;
#[path = "answer_review_query.rs"]
mod query;
pub use query::{held, read, ready};
/// Begin or update a user-requested answer-review hold.
pub async fn begin(session: &str, question: &str) -> anyhow::Result<bool> {
    codetether_goal_runtime::answer_review::begin::<super::journal::NativeJournal>(
        session, question,
    )
    .await
}
/// Record an explicit user decision for the matching goal and review.
pub async fn record(
    session: &str,
    goal_id: &str,
    review_id: &str,
    decision: AnswerReviewAction,
) -> anyhow::Result<()> {
    codetether_goal_runtime::answer_review::record::<super::journal::NativeJournal>(
        session, goal_id, review_id, decision,
    )
    .await
}
