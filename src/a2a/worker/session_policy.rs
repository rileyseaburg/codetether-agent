//! Standard session execution policy for worker tasks.

use std::sync::Arc;

use anyhow::Result;

use crate::{
    provider::{ContentPart, Message},
    session::Session,
};

use super::{
    AutoApprove, create_filtered_registry, load_task_provider_registry, run_session_steps,
    session_step_budget::resolve_step_budget,
};

mod session_model;

/// Result of a standard (non-swarm) worker session run.
pub(super) struct PolicySessionResult {
    pub result: crate::session::SessionResult,
    /// Set when the loop hit its step budget without the agent finishing.
    pub budget_exhausted: bool,
    pub max_steps: usize,
}

pub(super) async fn execute_session_with_policy(
    session: &mut Session,
    prompt: &str,
    auto_approve: AutoApprove,
    model_tier: Option<&str>,
    task_metadata: &serde_json::Map<String, serde_json::Value>,
    output_callback: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) -> Result<PolicySessionResult> {
    let registry = load_task_provider_registry(session).await?;
    let providers = registry.list();
    session_model::ensure_providers(&providers)?;
    let selection =
        session_model::select(session.metadata.model.as_deref(), &providers, model_tier)?;
    let provider = registry
        .get(&selection.provider)
        .ok_or_else(|| anyhow::anyhow!("Provider {} not found", selection.provider))?;
    session.add_delegated_message(Message {
        role: crate::provider::Role::User,
        content: vec![ContentPart::Text {
            text: prompt.to_string(),
        }],
    });
    if session.title.is_none() {
        session.generate_ai_title(&registry).await?;
    }
    let workspace_dir = session
        .metadata
        .directory
        .clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    let tool_registry = create_filtered_registry(
        Arc::clone(&provider),
        selection.model.clone(),
        auto_approve,
        &workspace_dir,
        output_callback.clone(),
    );
    let tool_definitions = tool_registry.definitions();
    tracing::info!(
        "Using model: {} via provider: {} (tier: {:?})",
        selection.model,
        selection.provider,
        model_tier
    );
    let text = run_session_steps(
        provider,
        session,
        &selection.model,
        &crate::agent::builtin::build_system_prompt(&workspace_dir),
        &tool_registry,
        &tool_definitions,
        auto_approve,
        &workspace_dir,
        output_callback,
        resolve_step_budget(task_metadata),
    )
    .await?;
    Ok(PolicySessionResult {
        result: crate::session::SessionResult {
            text: text.text,
            session_id: session.id.clone(),
        },
        budget_exhausted: text.budget_exhausted,
        max_steps: text.max_steps,
    })
}
