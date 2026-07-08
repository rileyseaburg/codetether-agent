//! `restore_step_model`: undo within-step provider failover each step.

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;

use super::step_restore::resolve_original;
use crate::provider::{Provider, ProviderRegistry, ToolDefinition};
use crate::session::Session;
use crate::session::helper::request_state::{ProviderStepState, build_provider_step_state};
use crate::tool::ToolRegistry;

/// Reset provider/model to the session's original selection when they drifted.
///
/// Reads `session.metadata.model` (written once before the step loop, never
/// updated by within-step failover) and restores all derived state when the
/// current values differ.
#[allow(clippy::too_many_arguments)]
pub(crate) fn restore_step_model(
    session: &Session,
    _providers: &[&str],
    registry: &ProviderRegistry,
    cwd: &PathBuf,
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
    let Some((orig_provider, orig_model)) = resolve_original(session, selected_provider, model)
    else {
        return Ok(());
    };
    tracing::info!(from_provider=%selected_provider, from_model=%model,
        to_provider=%orig_provider, to_model=%orig_model,
        "Restoring original model after within-step retry");
    *selected_provider = orig_provider.clone();
    *model = orig_model.clone();
    *provider = registry
        .get(&orig_provider)
        .ok_or_else(|| anyhow::anyhow!("Provider {orig_provider} not found"))?;
    *provider_state =
        build_provider_step_state(Arc::clone(provider), &orig_provider, &orig_model, cwd);
    provider_state.apply_to(
        tool_registry,
        tool_definitions,
        temperature,
        model_supports_tools,
        advertised_tool_definitions,
        system_prompt,
    );
    Ok(())
}
