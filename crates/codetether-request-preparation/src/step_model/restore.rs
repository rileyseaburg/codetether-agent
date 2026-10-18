use super::{Host, StepVars, original::resolve_original};
use std::sync::Arc;
/// Restore the original model and rebuild its derived fields if changed.
/// # Arguments
/// `vars` borrows active fields; `host` provides session metadata and providers.
/// # Returns
/// Success after restoration, or immediately if the selection already matches.
/// # Errors
/// Returns an error if the original provider is absent after identity assignment.
/// # Examples
/// ```no_run
/// # fn restore<R, S>(vars: &mut codetether_request_preparation::step_model::StepVars<'_, R, S>, host: &impl codetether_request_preparation::step_model::Host<R, S>) -> anyhow::Result<()> {
/// codetether_request_preparation::step_model::restore_step_model(vars, host)?;
/// # Ok(()) }
/// ```
pub fn restore_step_model<R, S>(
    vars: &mut StepVars<'_, R, S>,
    host: &impl Host<R, S>,
) -> anyhow::Result<()> {
    let Some((orig_provider, orig_model)) = resolve_original(
        host.original_model(vars.session),
        vars.selected_provider,
        vars.model,
    ) else {
        return Ok(());
    };
    tracing::info!(from_provider=%vars.selected_provider, from_model=%vars.model,
        to_provider=%orig_provider, to_model=%orig_model,
        "Restoring original model after within-step retry");
    *vars.selected_provider = orig_provider.clone();
    *vars.model = orig_model.clone();
    *vars.provider = host
        .provider(&orig_provider)
        .ok_or_else(|| anyhow::anyhow!("Provider {orig_provider} not found"))?;
    *vars.provider_state = host.build(
        Arc::clone(vars.provider),
        &orig_provider,
        &orig_model,
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
    Ok(())
}
