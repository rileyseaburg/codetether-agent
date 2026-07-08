//! Per-step begin hook: restore the original model, then apply proactive
//! rate-gate failover when the primary is near its RPM/TPM budget.

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;

use crate::provider::{Provider, ProviderRegistry, ToolDefinition};
use crate::session::Session;
use crate::session::helper::request_state::ProviderStepState;
use crate::session::rate_gate::check_rate_gate;
use crate::tool::ToolRegistry;

/// Restore the session's original model, then (optionally) swap to a rate-gate
/// substitute for this step when the primary model is near its budget.
#[allow(clippy::too_many_arguments)]
#[rustfmt::skip]
pub(crate) fn begin_step(
    session: &mut Session,
    providers: &[&str],
    registry: &ProviderRegistry,
    cwd: &PathBuf,
    rate_gate: bool,
    selected_provider: &mut String,
    model: &mut String,
    provider: &mut Arc<dyn Provider>,
    provider_state: &mut ProviderStepState,
    tool_registry: &mut Arc<ToolRegistry>,
    tool_definitions: &mut Vec<ToolDefinition>,
    temperature: &mut Option<f32>,
    model_supports_tools: &mut bool,
    advertised_tool_definitions: &mut Vec<ToolDefinition>,
    system_prompt: &mut String,
) -> Result<()> {
    super::step_model_restore::restore_step_model(
        session, providers, registry, cwd, selected_provider, model, provider,
        provider_state, tool_registry, tool_definitions, temperature,
        model_supports_tools, advertised_tool_definitions, system_prompt,
    )?;
    if !rate_gate {
        return Ok(());
    }
    let Some((sub_p, sub_m)) = check_rate_gate(selected_provider, model) else {
        return Ok(());
    };
    tracing::info!(substitute_provider = %sub_p, substitute_model = %sub_m,
        "Rate gate: using substitute for this step");
    super::step_model_restore::apply_failover(
        super::step_model_restore::StepVars {
            selected_provider, model, provider, provider_state, tool_registry,
            tool_definitions, advertised_tool_definitions, temperature,
            model_supports_tools, system_prompt, session, cwd,
        },
        sub_p, sub_m, registry,
    )
}
