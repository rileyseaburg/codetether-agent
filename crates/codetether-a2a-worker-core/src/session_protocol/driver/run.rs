//! Bounded worker step-loop and continuation sequencing.
use super::{Continuation, SessionLoopHost};
use crate::session_protocol::SessionStepsOutcome;
use anyhow::Result;

/// Execute model steps until completion or budget exhaustion.
///
/// # Arguments
/// * `host` - Application execution and persistence operations.
/// * `max_steps` - Maximum model steps allowed for this invocation.
/// # Returns
/// Trimmed output, budget-exhaustion status, and the configured budget.
/// # Errors
/// Propagates host control, execution, continuation, or final save errors.
/// # Examples
/// ```rust,no_run
/// # async fn example(host: &mut impl codetether_a2a_worker_core::session_protocol::driver::SessionLoopHost) -> anyhow::Result<()> {
/// let outcome = codetether_a2a_worker_core::session_protocol::driver::run(host, 50).await?;
/// assert_eq!(outcome.max_steps, 50);
/// # Ok(()) }
/// ```
pub async fn run<H: SessionLoopHost>(
    host: &mut H,
    max_steps: usize,
) -> Result<SessionStepsOutcome> {
    let mut output = String::new();
    let mut completed = false;
    for step in 1..=max_steps {
        host.before_step(step, max_steps).await?;
        if host.process_step(&mut output).await? {
            match host.continuation().await? {
                Continuation::Finish => {
                    completed = true;
                    break;
                }
                Continuation::Goal => output.clear(),
                Continuation::Steering => {}
            }
        }
    }
    if !completed {
        host.record_halt(&format!(
            "step budget ({max_steps}) exhausted before completion"
        ))
        .await;
    }
    host.save().await?;
    Ok(SessionStepsOutcome {
        text: output.trim().into(),
        budget_exhausted: !completed,
        max_steps,
    })
}
