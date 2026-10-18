//! Execute a selected provider with workspace tools and task budget.
use super::{Host, OutputCallback, PolicySessionResult, RunRequest};
use crate::{
    execution_policy::{AutoApprove, model_selection::ModelSelection},
    task_config::resolve_step_budget,
};
use codetether_provider_core::Provider;
use std::sync::Arc;
pub(super) async fn run(
    host: &mut impl Host,
    provider: Arc<dyn Provider>,
    selection: ModelSelection,
    auto_approve: AutoApprove,
    model_tier: Option<&str>,
    task_metadata: &serde_json::Map<String, serde_json::Value>,
    output_callback: OutputCallback,
) -> anyhow::Result<PolicySessionResult> {
    let workspace_dir = host
        .workspace()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    let tool_registry = host.tools(
        Arc::clone(&provider),
        selection.model.clone(),
        auto_approve,
        &workspace_dir,
        output_callback.clone(),
    );
    let tool_definitions = host.definitions(&tool_registry);
    tracing::info!(
        "Using model: {} via provider: {} (tier: {:?})",
        selection.model,
        selection.provider,
        model_tier
    );
    let text = host
        .run(RunRequest {
            provider,
            model: &selection.model,
            system_prompt: &host.system_prompt(&workspace_dir),
            tools: &tool_registry,
            definitions: &tool_definitions,
            auto_approve,
            workspace: &workspace_dir,
            output: output_callback,
            max_steps: resolve_step_budget(task_metadata),
        })
        .await?;
    Ok(super::result::assemble(text, host.session_id()))
}
