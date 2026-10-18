//! Persist explicit continuation before work can resume.
use codetether_goal_core::AnswerReviewAction;
/// Release a nonterminal goal's answer hold through a durable user decision.
/// # Arguments
/// `id` identifies the session whose goal is being continued.
/// # Returns
/// True when the goal may be continued, false for absent/terminal goals.
/// # Errors
/// Returns journal read or update errors.
/// # Examples
/// ```no_run
/// async fn release<J: codetether_goal_answer::Journal>() {
///     let _ = codetether_goal_answer::release::release::<J>("session").await;
/// }
/// ```
pub async fn release<J: crate::Journal>(id: &str) -> anyhow::Result<bool> {
    let state = J::read(id)?;
    if state
        .goal
        .as_ref()
        .is_none_or(|goal| goal.status.is_terminal())
    {
        return Ok(false);
    }
    if let Some(review) = state.answer_review {
        J::record(
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
