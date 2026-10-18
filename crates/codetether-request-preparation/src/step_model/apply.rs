use super::{Host, StepVars};
use std::sync::Arc;
/// Select an alternate provider/model, rebuild fields and prepare context.
/// # Arguments
/// `vars` borrows active fields; `retry_provider` and `retry_model` select the
/// target; `host` supplies provider lookup and preparation.
/// # Returns
/// Success after all fields and proactive context are updated.
/// # Errors
/// Returns an error for a missing provider after assigning the target identity.
/// # Examples
/// ```no_run
/// # fn switch<R, S>(vars: codetether_request_preparation::step_model::StepVars<'_, R, S>, host: &impl codetether_request_preparation::step_model::Host<R, S>) -> anyhow::Result<()> {
/// codetether_request_preparation::step_model::apply_failover(
///     vars, "provider".into(), "model".into(), host,
/// )?;
/// # Ok(()) }
/// ```
pub fn apply_failover<R, S>(
    vars: StepVars<'_, R, S>,
    retry_provider: String,
    retry_model: String,
    host: &impl Host<R, S>,
) -> anyhow::Result<()> {
    *vars.selected_provider = retry_provider;
    *vars.model = retry_model;
    *vars.provider = host
        .provider(vars.selected_provider)
        .ok_or_else(|| anyhow::anyhow!("Provider {} not found", vars.selected_provider))?;
    *vars.provider_state = host.build(
        Arc::clone(vars.provider),
        vars.selected_provider,
        vars.model,
        vars.session,
    );
    vars.provider_state.apply_to(
        vars.tool_registry,
        vars.tool_definitions,
        vars.temperature,
        vars.model_supports_tools,
        vars.advertised_tool_definitions,
        vars.system_prompt,
    );
    host.prepare(vars.session, Arc::clone(vars.provider), vars.model);
    Ok(())
}
