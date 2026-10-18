//! Prompt loop without event streaming.
use anyhow::Result;
use std::path::Path;
/// Run the initial prompt and at most the configured number of resume attempts.
/// # Arguments
/// * `session` - Session retaining transcript and checkpoint state.
/// * `message` - Original user objective.
/// * `max_steps` - Optional step budget.
/// * `resume_attempts` - Additional attempts allowed after exhaustion.
/// * `workspace` - Workspace recorded by checkpoints.
/// # Returns
/// The last prompt result, whether complete or still exhausted.
/// # Errors
/// Returns prompt, provider initialization or checkpoint errors.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_run_resume::RunSession>(session: &mut S) -> anyhow::Result<()> {
/// let result = codetether_run_resume::execute_prompt_with_resume(session, "task", None, Some(1), std::path::Path::new(".")).await?;
/// # Ok(()) }
/// ```
pub async fn execute_prompt_with_resume<S: crate::RunSession>(
    session: &mut S,
    message: &str,
    max_steps: Option<usize>,
    resume_attempts: Option<usize>,
    workspace: &Path,
) -> Result<codetether_session_events::SessionResult> {
    let mut prompt_text = message.to_string();
    let mut resumes_left = resume_attempts.unwrap_or(0);
    loop {
        let before = session.messages().len();
        let result = session.prompt(&prompt_text).await?;
        if !codetether_step_limit::was_budget_exhausted() {
            codetether_run_checkpoint::storage::clear(session).await?;
            return Ok(result);
        }
        let budget = max_steps.unwrap_or(S::default_max_steps());
        if crate::checkpoint::should_checkpoint(before, session, budget) {
            crate::checkpoint::persist_exhaustion_checkpoint(session, message, budget, workspace)
                .await?;
        }
        if resumes_left == 0 {
            return Ok(result);
        }
        resumes_left -= 1;
        let Some(plan) = crate::checkpoint::resume_plan(session, resumes_left).await? else {
            return Ok(result);
        };
        session.set_max_steps(Some(budget));
        prompt_text = plan.prompt;
    }
}
