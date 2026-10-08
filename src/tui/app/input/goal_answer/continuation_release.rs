//! Persist human continuation before allowing the runtime to resume work.
use crate::session::tasks::{AnswerReviewAction, runtime::answer_review};
#[cfg(test)]
#[path = "continuation_release_tests.rs"]
mod tests;

pub(super) async fn release(id: &str) -> anyhow::Result<bool> {
    let state = answer_review::read(id)?;
    if state
        .goal
        .as_ref()
        .is_none_or(|goal| goal.status.is_terminal())
    {
        return Ok(false);
    }
    if let Some(review) = state.answer_review {
        answer_review::record(
            id,
            &review.goal_id,
            &review.id,
            AnswerReviewAction::ResumeRequested,
        )
        .await?;
        return Ok(true);
    }
    Ok(state.goal.is_some_and(|goal| goal.status.is_active()))
}
