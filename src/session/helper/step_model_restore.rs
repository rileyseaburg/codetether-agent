//! Provider failover and step-model restore helpers.
//!
//! `apply_failover` swaps to a retry provider/model and rebuilds all derived
//! state. `restore_step_model` (in `step_restore`) resets provider/model back
//! to the session's original selection at the start of each step.

use std::sync::Arc;
use anyhow::Result;
use crate::provider::ProviderRegistry;
use super::request_state::build_provider_step_state;

#[path = "step_vars.rs"]
mod step_vars;
#[path = "step_restore.rs"]
mod step_restore;

pub(super) use step_vars::StepVars;
pub(super) use step_restore::restore_step_model;

/// Swap to `retry_provider`/`retry_model` and rebuild all derived state.
pub(super) fn apply_failover(
    vars: StepVars<'_>,
    retry_provider: String,
    retry_model: String,
    registry: &ProviderRegistry,
) -> Result<()> {
    *vars.selected_provider = retry_provider;
    *vars.model = retry_model;
    *vars.provider = registry
        .get(vars.selected_provider)
        .ok_or_else(|| anyhow::anyhow!("Provider {} not found", vars.selected_provider))?;
    *vars.provider_state = build_provider_step_state(
        Arc::clone(vars.provider), vars.selected_provider, vars.model, vars.cwd,
    );
    vars.provider_state.apply_to(
        vars.tool_registry, vars.tool_definitions, vars.temperature,
        vars.model_supports_tools, vars.advertised_tool_definitions, vars.system_prompt,
    );
    Ok(())
}
