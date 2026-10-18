//! Native session specialization of user-only answer-review operations.
use crate::session::tasks::AnswerReviewAction;
#[path = "answer_review_query.rs"]
mod query;
pub(crate) use query::{held, read, ready};
pub(crate) async fn begin(session: &str, question: &str) -> anyhow::Result<bool> {
    codetether_goal_runtime::answer_review::begin::<super::journal::NativeJournal>(
        session, question,
    )
    .await
}
pub(crate) async fn record(
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
#[cfg(test)]
#[path = "answer_review_tests.rs"]
mod tests;
