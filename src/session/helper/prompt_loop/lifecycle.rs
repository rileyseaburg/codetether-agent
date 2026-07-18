//! Iteration and per-step preparation for the shared prompt loop.

use super::state::{Runner, StepFlow};
use crate::session::{SessionEvent, SessionResult};
use anyhow::Result;

/// Runs provider/tool steps and returns the persisted session result.
///
/// # Errors
///
/// Returns an error from setup, completion, tool execution, or persistence.
pub(crate) async fn run(runner: &mut Runner<'_>) -> Result<SessionResult> {
    let _steering = super::super::steering::RunGuard::open(&runner.session.id);
    let mut step = 0;
    loop {
        step += 1;
        begin(runner, step).await?;
        let Some(response) = super::turn_completion::next(runner, step).await? else {
            step = 0;
            continue;
        };
        let response = super::response::normalize(runner, response).await;
        match super::response::handle(runner, step, response).await? {
            StepFlow::Finish => break,
            StepFlow::ContinueGoal => step = 0,
            StepFlow::Continue if step >= runner.progress.max_steps => {
                match super::response::continue_goal(runner).await {
                    StepFlow::ContinueGoal => step = 0,
                    StepFlow::Finish | StepFlow::Continue => break,
                }
            }
            StepFlow::Continue => {}
        }
    }
    crate::session::step_limit::clear_budget();
    super::finish::finish(runner).await
}

async fn begin(runner: &mut Runner<'_>, step: usize) -> Result<()> {
    super::super::steering::drain_into(runner.session);
    let model = &mut runner.model;
    #[rustfmt::skip]
    super::super::step_begin::begin_step(super::super::step_model_restore::StepVars {
        selected_provider: &mut model.provider_name, model: &mut model.model_id,
        provider: &mut model.provider, provider_state: &mut model.step,
        tool_registry: &mut model.registry, tool_definitions: &mut model.tools,
        advertised_tool_definitions: &mut model.advertised,
        temperature: &mut model.temperature, model_supports_tools: &mut model.supports_tools,
        system_prompt: &mut model.system_prompt, session: runner.session,
    }, &runner.registry, runner.events.is_some())?;
    tracing::info!(step, "Agent step starting");
    runner.subagents.inject(runner.session);
    if let Some(tx) = &runner.events {
        let _ = tx.send(SessionEvent::Thinking).await;
    }
    super::super::cost_guard::enforce_cost_budget()
}
