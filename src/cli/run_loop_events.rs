//! Event-streaming prompt loop for resumable `codetether run`.

use crate::cli::run_checkpoint;
use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent, SessionResult};
use anyhow::Result;
use std::{path::Path, sync::Arc};
use tokio::sync::mpsc;

pub async fn execute_prompt_with_resume_events(
    session: &mut Session,
    message: &str,
    max_steps: Option<usize>,
    resume_attempts: Option<usize>,
    workspace: &Path,
    event_tx: mpsc::Sender<SessionEvent>,
) -> Result<SessionResult> {
    let registry = ProviderRegistry::shared_from_vault().await?;
    let mut prompt_text = message.to_string();
    let mut resumes_left = resume_attempts.unwrap_or(0);
    loop {
        let before = session.messages.len();
        let result = session
            .prompt_with_events(&prompt_text, event_tx.clone(), Arc::clone(&registry))
            .await?;
        if !crate::session::step_limit::was_budget_exhausted() {
            session.clear_run_checkpoint().await?;
            return Ok(result);
        }
        let budget = max_steps.unwrap_or(crate::session::DEFAULT_MAX_STEPS);
        if run_checkpoint::should_checkpoint(before, session, budget) {
            run_checkpoint::persist_exhaustion_checkpoint(session, message, budget, workspace)
                .await?;
        }
        if resumes_left == 0 {
            return Ok(result);
        }
        resumes_left -= 1;
        let Some(plan) = run_checkpoint::resume_plan(session, resumes_left).await? else {
            return Ok(result);
        };
        session.max_steps = Some(budget);
        prompt_text = plan.prompt;
    }
}
