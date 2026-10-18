use super::Host;
use crate::StepFlow;
use codetether_session_events::SessionResult;
/// Run provider and tool steps until completion or an exhausted step budget.
/// # Arguments
/// `host` supplies completion, recovery and native lifecycle effects.
/// # Returns
/// The persisted session result produced by the native finalizer.
/// # Errors
/// Propagates preparation, completion, response, continuation and persistence failures.
/// # Examples
/// ```no_run
/// # async fn execute(host: &mut impl codetether_prompt_loop::lifecycle::Host) -> anyhow::Result<()> {
/// let _result = codetether_prompt_loop::lifecycle::run(host).await?;
/// # Ok(()) }
/// ```
pub async fn run(host: &mut impl Host) -> anyhow::Result<SessionResult> {
    let mut step = 0;
    let budget_exhausted = loop {
        step += 1;
        host.begin(step).await?;
        let Some(response) = super::completion::next(host, step).await? else {
            step = 0;
            continue;
        };
        let response = host.normalize(response).await;
        match host.handle(step, response).await? {
            StepFlow::Finish => break false,
            StepFlow::ContinueGoal => step = 0,
            StepFlow::Continue
                if super::budget::exhausted(
                    step,
                    crate::goal_recovery::Host::progress(host).max_steps,
                ) =>
            {
                match crate::goal_recovery::Host::continue_goal(host).await? {
                    StepFlow::ContinueGoal => step = 0,
                    StepFlow::Finish | StepFlow::Continue => break true,
                }
            }
            StepFlow::Continue => {}
        }
    };
    if !budget_exhausted {
        host.clear_budget();
    }
    host.finish().await
}
