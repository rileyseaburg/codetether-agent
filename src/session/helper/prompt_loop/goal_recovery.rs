//! Whole-turn continuation after exhausted retryable provider failures.

#[path = "goal_recovery/notify.rs"]
mod notify;
#[path = "goal_recovery/signature.rs"]
mod signature;
#[path = "goal_recovery/terminal.rs"]
mod terminal;

use super::{Runner, state::StepFlow};

const MAX_FAILURE_TURNS: u8 = 3;

pub(super) async fn after(runner: &mut Runner<'_>, error: &anyhow::Error) -> bool {
    if !super::super::error::is_retryable_upstream_error(error) || !active(runner).await {
        return false;
    }
    record(runner, error);
    let attempt = runner.progress.goal_failure_repeats;
    if attempt >= MAX_FAILURE_TURNS {
        terminal::persist(runner).await;
        return false;
    }
    notify::send(runner, error, attempt, MAX_FAILURE_TURNS - 1).await;
    tokio::time::sleep(std::time::Duration::from_secs(u64::from(attempt))).await;
    matches!(
        super::response::continue_goal(runner).await,
        StepFlow::ContinueGoal
    )
}

async fn active(runner: &Runner<'_>) -> bool {
    crate::session::tasks::runtime::current(&runner.session.id)
        .await
        .ok()
        .and_then(|(_, state)| state.goal)
        .is_some_and(|goal| goal.status.is_active())
}

fn record(runner: &mut Runner<'_>, error: &anyhow::Error) {
    let signature = signature::of(error);
    if runner.progress.goal_failure_signature.as_ref() == Some(&signature) {
        runner.progress.goal_failure_repeats += 1;
    } else {
        runner.progress.goal_failure_signature = Some(signature);
        runner.progress.goal_failure_repeats = 1;
    }
}
