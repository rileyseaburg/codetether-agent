//! `restore_step_model`: undo within-step provider failover each step.

use std::sync::Arc;

use anyhow::Result;

use super::step_restore::resolve_original;
use super::step_vars::StepVars;
use crate::provider::ProviderRegistry;
use crate::session::helper::request_state::build_provider_step_state;

/// Reset provider/model to the session's original selection when they drifted.
///
/// Reads `session.metadata.model` (written once before the step loop, never
/// updated by within-step failover) and restores all derived state when the
/// current values differ.
pub(crate) fn restore_step_model(
    vars: &mut StepVars<'_>,
    registry: &ProviderRegistry,
) -> Result<()> {
    let Some((orig_provider, orig_model)) =
        resolve_original(vars.session, vars.selected_provider, vars.model)
    else {
        return Ok(());
    };
    tracing::info!(from_provider=%vars.selected_provider, from_model=%vars.model,
        to_provider=%orig_provider, to_model=%orig_model,
        "Restoring original model after within-step retry");
    *vars.selected_provider = orig_provider.clone();
    *vars.model = orig_model.clone();
    *vars.provider = registry
        .get(&orig_provider)
        .ok_or_else(|| anyhow::anyhow!("Provider {orig_provider} not found"))?;
    *vars.provider_state = build_provider_step_state(
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
