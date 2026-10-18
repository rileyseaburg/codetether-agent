//! Durable user-only journal operations for shared answer workflows.
use crate::session::tasks::{AnswerReviewAction, TaskState, runtime::answer_review};
pub(super) struct Native;
impl codetether_goal_answer::Journal for Native {
    fn read(id: &str) -> anyhow::Result<TaskState> {
        answer_review::read(id)
    }
    fn held(id: &str) -> bool {
        answer_review::held(id)
    }
    async fn begin(id: &str, question: &str) -> anyhow::Result<bool> {
        answer_review::begin(id, question).await
    }
    async fn record(
        id: &str,
        goal: &str,
        review: &str,
        action: AnswerReviewAction,
    ) -> anyhow::Result<()> {
        answer_review::record(id, goal, review, action).await
    }
}
