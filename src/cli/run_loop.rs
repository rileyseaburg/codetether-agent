//! Prompt execution loop for resumable `codetether run`.

use crate::session::Session;
use anyhow::Result;
use std::path::Path;

pub async fn execute_prompt_with_resume(
    session: &mut Session,
    message: &str,
    max_steps: Option<usize>,
    attempts: Option<usize>,
    workspace: &Path,
) -> Result<crate::session::SessionResult> {
    let mut prompt_text = message.to_string();
    let mut attempts_left = attempts.unwrap_or(1);
    loop {
        let before = session.messages.len();
        let result = session.prompt(&prompt_text).await?;
        if !crate::session::step_limit::was_budget_exhausted() {
            session.clear_run_checkpoint().await?;
            return Ok(result);
        }
        let budget = max_steps.unwrap_or(crate::session::DEFAULT_MAX_STEPS);
        if super::run_checkpoint::should_checkpoint(before, session, budget) {
            super::run_checkpoint::persist_exhaustion_checkpoint(
                session, message, budget, workspace,
            )
            .await?;
        }
        attempts_left = attempts_left.saturating_sub(1);
        if attempts_left == 0 {
            return Ok(result);
        }
        let Some(plan) = super::run_checkpoint::resume_plan(session, attempts_left).await? else {
            return Ok(result);
        };
        session.max_steps = Some(budget);
        prompt_text = plan.prompt;
    }
}
