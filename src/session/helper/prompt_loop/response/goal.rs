//! Final-answer routing for active goal continuation.

use super::super::{Runner, state::StepFlow};

pub(super) async fn flow(runner: &mut Runner<'_>) -> anyhow::Result<StepFlow> {
    let decision = crate::session::runtime::continuation::continue_goal(runner.session).await?;
    Ok(apply(runner, decision))
}

pub(super) async fn after_message(runner: &mut Runner<'_>) -> anyhow::Result<StepFlow> {
    let decision = crate::session::runtime::continuation::after_message(runner.session).await?;
    Ok(apply(runner, decision))
}

fn apply(
    runner: &mut Runner<'_>,
    decision: crate::session::runtime::continuation::Continuation,
) -> StepFlow {
    use crate::session::runtime::continuation::Continuation;
    match decision {
        Continuation::Goal => {
            reset_turn(runner);
            StepFlow::ContinueGoal
        }
        Continuation::Steering => StepFlow::Continue,
        Continuation::Finish => StepFlow::Finish,
    }
}
fn reset_turn(runner: &mut Runner<'_>) {
    runner.progress.reset_goal_turn();
}
