//! Preparation performed before each prompt-loop provider step.

use super::super::state::Runner;
use crate::session::SessionEvent;
use anyhow::Result;

pub(super) async fn run(runner: &mut Runner<'_>, step: usize) -> Result<()> {
    super::super::super::steering::drain_into(runner.session);
    let model = &mut runner.model;
    #[rustfmt::skip]
    super::super::super::step_begin::begin_step(super::super::super::step_model_restore::StepVars {
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
    super::super::super::cost_guard::enforce_cost_budget()
}
