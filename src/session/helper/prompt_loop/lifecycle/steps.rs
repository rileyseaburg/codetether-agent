//! Iterative provider completion and response-flow control.

use super::super::state::{Runner, StepFlow};
use crate::session::SessionResult;
use anyhow::Result;

pub(super) async fn run(runner: &mut Runner<'_>) -> Result<SessionResult> {
    let mut step = 0;
    let budget_exhausted = loop {
        step += 1;
        super::begin::run(runner, step).await?;
        let Some(response) = super::super::turn_completion::next(runner, step).await? else {
            step = 0;
            continue;
        };
        let response = super::super::response::normalize(runner, response).await;
        match super::super::response::handle(runner, step, response).await? {
            StepFlow::Finish => break false,
            StepFlow::ContinueGoal => step = 0,
            StepFlow::Continue if super::budget::exhausted(step, runner.progress.max_steps) => {
                match super::super::response::continue_goal(runner).await {
                    StepFlow::ContinueGoal => step = 0,
                    StepFlow::Finish | StepFlow::Continue => break true,
                }
            }
            StepFlow::Continue => {}
        }
    };
    if !budget_exhausted {
        crate::session::step_limit::clear_budget();
    }
    super::super::finish::finish(runner).await
}
