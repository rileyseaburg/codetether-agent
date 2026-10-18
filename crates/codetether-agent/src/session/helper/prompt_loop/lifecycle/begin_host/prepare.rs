//! Borrow the native model fields for step preparation.
use super::Runner;
pub(super) fn run(runner: &mut Runner<'_>) -> anyhow::Result<()> {
    let model = &mut runner.inner.model;
    #[rustfmt::skip]
    super::super::super::super::super::step_begin::begin_step(super::super::super::super::super::step_model_restore::StepVars {
        selected_provider: &mut model.provider_name, model: &mut model.model_id,
        provider: &mut model.provider, provider_state: &mut model.step,
        tool_registry: &mut model.registry, tool_definitions: &mut model.tools,
        advertised_tool_definitions: &mut model.advertised,
        temperature: &mut model.temperature, model_supports_tools: &mut model.supports_tools,
        system_prompt: &mut model.system_prompt, session: runner.inner.session,
    }, &runner.inner.registry, runner.inner.events.is_some())?;
    Ok(())
}
